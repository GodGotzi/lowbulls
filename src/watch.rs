//! This module provides a LowBullWatcher structure for tracking history of watched entries.
//!
//! # Examples
//!
//! ```
//! use std::time::Instant;
//! use lowbulls::watch::LowBullWatcher;
//!
//! let mut watcher = LowBullWatcher::new(5);
//!
//! watcher.watch("entry1");
//! watcher.watch("entry2");
//!
//! watcher.debug_history();
//! ```
//!
//! # Notes
//!
//! - LowBullWatcher keeps track of the history of watched entries along with their timestamps.
//! - It maintains a fixed-size history buffer, dropping oldest entries when the buffer is full.
//!
use std::time::Instant;

/// Represents an entry being watched, holding a key and its creation time.
#[derive(Clone)]
pub struct WatchEntry<K: std::fmt::Debug + Clone> {
    /// The key of the entry being watched.
    key: K,
    /// The time when the entry was created.
    time: Instant,
}

impl<K: std::fmt::Debug + Clone> WatchEntry<K> {
    /// Constructs a new WatchEntry with the given key and current timestamp.
    pub fn new(key: K) -> Self {
        Self {
            key,
            time: Instant::now(),
        }
    }
}

impl<K: std::fmt::Debug + Clone> std::fmt::Debug for WatchEntry<K> {
    /// Formats the WatchEntry for debug purposes.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WatchEntry")
            .field("key", &self.key)
            .field("time", &self.time)
            .finish()
    }
}

/// Watches entries and maintains a history of watched entries.
pub struct LowBullWatcher<K: std::fmt::Debug + Clone> {
    /// The maximum size of the history buffer.
    floating_index: usize,
    /// The history buffer storing watched entries.
    history: Vec<Option<WatchEntry<K>>>,
}

impl<K: std::fmt::Debug + Clone> LowBullWatcher<K> {
    /// Constructs a new LowBullWatcher with the specified history size.
    pub fn new(history_size: usize) -> Self {
        Self {
            floating_index: 0,
            history: vec![None; history_size],
        }
    }

    /// Adds a new entry to the watch history, dropping oldest entry if history is full.
    pub fn watch(&mut self, key: K) {
        if self.floating_index == self.history.len() {
            for i in 0..(self.history.len() - 1) {
                let after = self.history[i].take();
                self.history[i] = self.history[i + 1].take();
                self.history[i + 1] = after;
            }
        }

        if self.floating_index < self.history.len() {
            self.floating_index += 1;
        }

        self.history[self.floating_index - 1] = Some(WatchEntry::new(key.clone()));
    }

    /// Prints the debug information of all entries in the watch history.
    pub fn debug_history(&self) {
        for entry in &self.history {
            println!("{:?}", entry);
        }
    }
}
