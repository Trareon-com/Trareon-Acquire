//! Presentation helpers for the Acquire Slint shell (testable without a display).

/// Snapshot shown in the foundation Slint window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiSnapshot {
    pub status_line: String,
    pub last_package: String,
    pub busy: bool,
}

impl Default for UiSnapshot {
    fn default() -> Self {
        Self {
            status_line: "Ready — Lab Use Only (Engineering Alpha)".into(),
            last_package: "(none)".into(),
            busy: false,
        }
    }
}

impl UiSnapshot {
    pub fn set_busy(&mut self, busy: bool) {
        self.busy = busy;
    }

    pub fn set_ok(&mut self, package: impl Into<String>, detail: impl Into<String>) {
        self.busy = false;
        self.last_package = package.into();
        self.status_line = detail.into();
    }

    pub fn set_err(&mut self, detail: impl Into<String>) {
        self.busy = false;
        self.status_line = detail.into();
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_ok_and_clear() {
        let mut snap = UiSnapshot::default();
        snap.set_busy(true);
        assert!(snap.busy);
        snap.set_ok("/tmp/demo.fsnap", "VALID");
        assert!(!snap.busy);
        assert_eq!(snap.last_package, "/tmp/demo.fsnap");
        snap.clear();
        assert_eq!(snap, UiSnapshot::default());
    }
}
