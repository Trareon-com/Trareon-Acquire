//! Lab HPA/DCO probe (Live Gate Part 1).
//!
//! Prints reported size + detection status. Does **not** unlock HPA.
//! On platforms without ATA passthrough, status is `Unavailable` with a reason
//! (fail-soft — exit 0 so operators can still record limitations).
//!
//! ```bash
//! # File / synthetic path (CI-safe):
//! cargo run -p trareon-ata --example lab_hpa_dco_probe -- /tmp/some.bin
//!
//! # Lab disk (elevated as needed):
//! sudo cargo run -p trareon-ata --example lab_hpa_dco_probe -- /dev/rdisk10
//! ```

use std::{env, path::PathBuf, process};
use trareon_ata::{DetectionStatus, detect_hpa_dco, probe_write_blocker};

fn main() {
    let path = PathBuf::from(env::args().nth(1).unwrap_or_else(|| "/dev/rdisk10".into()));

    let wb = probe_write_blocker(&path);
    println!("WRITE_BLOCKER {wb:?}");

    match detect_hpa_dco(&path) {
        Ok(r) => {
            println!(
                "HPA_DCO reported={} accessible={} hpa={} dco={} native_max={:?} dco_max={:?}",
                r.reported_size,
                r.accessible_size,
                r.hpa_detected,
                r.dco_detected,
                r.hpa_native_max,
                r.dco_max
            );
            match &r.status {
                DetectionStatus::Ok => println!("STATUS Ok"),
                DetectionStatus::Unavailable { reason } => {
                    println!("STATUS Unavailable reason={reason}");
                    println!("NOTE: do not claim full-disk until ATA probe is Ok on this OS");
                }
            }
            if r.is_hidden_area() {
                println!("WARNING: hidden area indicated — acquire coverage may be incomplete");
            }
        }
        Err(e) => {
            eprintln!("HPA_DCO_FAIL {e}");
            process::exit(1);
        }
    }
}
