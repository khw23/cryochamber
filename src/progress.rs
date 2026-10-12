//! Rate limiting for ordinary, agent-authored scientific syntheses only.
//! No autonomous delivery: a later agent must refresh the pending synthesis.
use anyhow::Result;
use chrono::{NaiveDateTime, Timelike};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Default, Serialize, Deserialize)]
pub struct ProgressState {
    pub last_sent: Option<NaiveDateTime>,
    pub last_text: Option<String>,
    pub pending: Option<String>,
}

impl ProgressState {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        Ok(serde_json::from_slice(&std::fs::read(path)?)?)
    }

    pub fn due(&self, now: NaiveDateTime, interval: u64, text: &str) -> bool {
        // Local daemon clock: ordinary progress only, never human replies/alerts.
        if now.hour() < 8 || now.hour() >= 22 {
            return false;
        }
        if self.last_text.as_deref().map(str::trim) == Some(text.trim()) {
            return false;
        }
        self.last_sent.is_none_or(|last| {
            (now - last).num_seconds() >= 0 && (now - last).num_seconds() as u64 >= interval
        })
    }

    pub fn delivered(&mut self, now: NaiveDateTime, text: String) {
        self.last_sent = Some(now);
        self.last_text = Some(text);
        self.pending = None;
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, serde_json::to_vec(self)?)?;
        std::fs::rename(tmp, path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn at(s: &str) -> NaiveDateTime {
        s.parse().unwrap()
    }

    #[test]
    fn ordinary_progress_is_quiet_at_night_and_limited_across_restart() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("progress.json");
        let mut state = ProgressState::default();
        assert!(!state.due(at("2026-10-11T23:00:00"), 86400, "finding"));
        assert!(state.due(at("2026-10-12T09:00:00"), 86400, "finding"));
        state.delivered(at("2026-10-12T09:00:00"), "finding".into());
        state.save(&path).unwrap();
        let state = ProgressState::load(&path).unwrap();
        assert!(!state.due(at("2026-10-12T10:00:00"), 86400, "new finding"));
        assert!(!state.due(at("2026-10-13T09:00:00"), 86400, "finding\n"));
        assert!(state.due(at("2026-10-13T09:00:00"), 86400, "new finding"));
    }
}
