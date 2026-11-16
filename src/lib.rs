// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-rclone: rclone component model.
//!
//! Real implementation: rclone_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RcloneItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RcloneError {
    EmptyRcloneItemName,
    DuplicateRcloneItem(String),
    EmptyEventSubject,
}

impl fmt::Display for RcloneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRcloneItemName => write!(f, "rclone_item name must not be empty"),
            Self::DuplicateRcloneItem(id) => write!(f, "duplicate rclone_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for RcloneError {}

pub struct Rclone {
    pub name: String,
    rclone_items: BTreeMap<String, RcloneItem>,
    events: BTreeMap<String, Event>,
}

impl Rclone {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            rclone_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_rclone_item(&mut self, item: RcloneItem) -> Result<(), RcloneError> {
        if item.name.trim().is_empty() {
            return Err(RcloneError::EmptyRcloneItemName);
        }
        let id = item.id.clone();
        if self.rclone_items.contains_key(&id) {
            return Err(RcloneError::DuplicateRcloneItem(id.clone()));
        }
        self.rclone_items.insert(id, item);
        Ok(())
    }

    pub fn rclone_item_count(&self) -> usize {
        self.rclone_items.len()
    }

    pub fn get_rclone_item(&self, id: &str) -> Option<&RcloneItem> {
        self.rclone_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), RcloneError> {
        if event.subject.trim().is_empty() {
            return Err(RcloneError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_rclone_item(id: &str, name: &str) -> RcloneItem {
        RcloneItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Rclone::new("Rclone");
        assert_eq!(m.name, "Rclone");
        assert_eq!(m.rclone_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_rclone_item() {
        let mut m = Rclone::new("Rclone");
        m.add_rclone_item(sample_rclone_item("a1", "Alpha")).unwrap();
        assert_eq!(m.rclone_item_count(), 1);
        assert!(m.get_rclone_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Rclone::new("Rclone");
        assert!(m.add_rclone_item(sample_rclone_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Rclone::new("Rclone");
        m.add_rclone_item(sample_rclone_item("x", "One")).unwrap();
        assert!(m.add_rclone_item(sample_rclone_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Rclone::new("Rclone");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
