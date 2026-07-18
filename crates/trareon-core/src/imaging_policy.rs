//! Bad-sector / retry policy recorded into custody (Wave 2).
//!
//! Live skip/retry I/O for physical media remains lab-gated. This module makes
//! the operator choice explicit and auditable before an acquire starts.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BadSectorPolicy {
    /// Skip unreadable ranges, zero-fill or mark gap, continue (lab default).
    #[default]
    SkipAndLog,
    /// Retry each failing read up to `retries` times, then skip.
    RetryThenSkip { retries: u32 },
    /// Stop the acquisition on the first unreadable range.
    FailClosed,
}

impl BadSectorPolicy {
    pub fn from_index(index: i32) -> Self {
        match index {
            1 => Self::RetryThenSkip { retries: 3 },
            2 => Self::FailClosed,
            _ => Self::SkipAndLog,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::SkipAndLog => "skip & log (lab default)",
            Self::RetryThenSkip { .. } => "retry ×3 then skip",
            Self::FailClosed => "fail-closed on bad sector",
        }
    }

    pub fn custody_note(self) -> String {
        format!("imaging_policy={}", self.label())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_maps_and_labels() {
        assert_eq!(BadSectorPolicy::from_index(0), BadSectorPolicy::SkipAndLog);
        assert!(matches!(
            BadSectorPolicy::from_index(1),
            BadSectorPolicy::RetryThenSkip { retries: 3 }
        ));
        assert_eq!(BadSectorPolicy::from_index(2), BadSectorPolicy::FailClosed);
        assert!(
            BadSectorPolicy::FailClosed
                .custody_note()
                .contains("fail-closed")
        );
    }
}
