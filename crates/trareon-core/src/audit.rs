use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{AcquisitionId, AcquisitionState, CoreError};

const GENESIS_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const MAX_FOUNDATION_EVENTS: usize = 100_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub sequence: u64,
    pub acquisition_id: AcquisitionId,
    pub timestamp_utc: DateTime<Utc>,
    pub state: AcquisitionState,
    pub action: String,
    pub previous_hash: String,
    pub event_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EventPayload<'a> {
    sequence: u64,
    acquisition_id: AcquisitionId,
    timestamp_utc: DateTime<Utc>,
    state: AcquisitionState,
    action: &'a str,
    previous_hash: &'a str,
}

fn hash_payload(payload: &EventPayload<'_>) -> Result<String, CoreError> {
    let bytes =
        serde_json::to_vec(payload).map_err(|error| CoreError::Serialization(error.to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(payload.previous_hash.as_bytes());
    hasher.update(&bytes);
    Ok(hex::encode(hasher.finalize()))
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditJournal {
    events: Vec<AuditEvent>,
}

impl AuditJournal {
    pub fn append(
        &mut self,
        acquisition_id: AcquisitionId,
        timestamp_utc: DateTime<Utc>,
        state: AcquisitionState,
        action: impl Into<String>,
    ) -> Result<&AuditEvent, CoreError> {
        let action = action.into();
        if action.is_empty() {
            return Err(CoreError::Verification(
                "audit action must not be empty".to_string(),
            ));
        }

        let sequence = self.events.len() as u64;
        let previous_hash = self
            .events
            .last()
            .map(|event| event.event_hash.clone())
            .unwrap_or_else(|| GENESIS_HASH.to_string());

        let payload = EventPayload {
            sequence,
            acquisition_id,
            timestamp_utc,
            state,
            action: &action,
            previous_hash: &previous_hash,
        };
        let event_hash = hash_payload(&payload)?;

        self.events.push(AuditEvent {
            sequence,
            acquisition_id,
            timestamp_utc,
            state,
            action,
            previous_hash,
            event_hash,
        });

        Ok(self.events.last().expect("event was just pushed"))
    }

    pub fn verify(&self) -> Result<String, CoreError> {
        let mut expected_previous = GENESIS_HASH.to_string();
        for (index, event) in self.events.iter().enumerate() {
            if event.sequence != index as u64 {
                return Err(CoreError::Verification(format!(
                    "sequence discontinuity at index {index}: expected {index}, found {}",
                    event.sequence
                )));
            }
            if event.previous_hash != expected_previous {
                return Err(CoreError::Verification(format!(
                    "previous-hash mismatch at sequence {}",
                    event.sequence
                )));
            }

            let payload = EventPayload {
                sequence: event.sequence,
                acquisition_id: event.acquisition_id,
                timestamp_utc: event.timestamp_utc,
                state: event.state,
                action: &event.action,
                previous_hash: &event.previous_hash,
            };
            let recomputed = hash_payload(&payload)?;
            if recomputed != event.event_hash {
                return Err(CoreError::Verification(format!(
                    "event hash mismatch at sequence {}",
                    event.sequence
                )));
            }

            expected_previous = event.event_hash.clone();
        }

        Ok(expected_previous)
    }

    pub fn write_jsonl(&self, path: &Path) -> Result<(), CoreError> {
        let mut file = File::create(path).map_err(|error| CoreError::Io(error.to_string()))?;
        for event in &self.events {
            let line = serde_json::to_string(event)
                .map_err(|error| CoreError::Serialization(error.to_string()))?;
            file.write_all(line.as_bytes())
                .map_err(|error| CoreError::Io(error.to_string()))?;
            file.write_all(b"\n")
                .map_err(|error| CoreError::Io(error.to_string()))?;
        }
        file.sync_all()
            .map_err(|error| CoreError::Io(error.to_string()))?;
        Ok(())
    }

    pub fn read_jsonl(path: &Path) -> Result<Self, CoreError> {
        let file = File::open(path).map_err(|error| CoreError::Io(error.to_string()))?;
        let reader = BufReader::new(file);

        let mut events = Vec::new();
        for line in reader.lines() {
            let line = line.map_err(|error| CoreError::Io(error.to_string()))?;
            if line.trim().is_empty() {
                return Err(CoreError::Verification(
                    "audit journal contains a blank record".to_string(),
                ));
            }

            let mut deserializer = serde_json::Deserializer::from_str(&line);
            let event = AuditEvent::deserialize(&mut deserializer)
                .map_err(|error| CoreError::Serialization(error.to_string()))?;
            deserializer.end().map_err(|_| {
                CoreError::Serialization("trailing data in audit record".to_string())
            })?;

            events.push(event);

            if events.len() > MAX_FOUNDATION_EVENTS {
                return Err(CoreError::Verification(
                    "audit journal exceeds foundation event limit".to_string(),
                ));
            }
        }

        if events.is_empty() {
            return Err(CoreError::Verification(
                "audit journal is empty".to_string(),
            ));
        }

        let journal = AuditJournal { events };
        journal.verify()?;
        Ok(journal)
    }

    #[doc(hidden)]
    pub fn events_mut_for_test(&mut self) -> &mut [AuditEvent] {
        &mut self.events
    }
}
