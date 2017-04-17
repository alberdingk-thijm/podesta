//! ## Example
//!
//! ### Serializing and deserializing an Event
//! Uses serde_json
//!
//! ```
//! # extern crate serde_json;
//! # extern crate podesta;
//! # fn main() {
//! use podesta::events;
//! let ev = events::Event {
//!     name: "Fire".to_string(),
//!     id: 1,
//!     desc: "%s has erupted into flames!".to_string(),
//!     chance: 3,
//!     effects: vec!(),
//! };
//! let se = serde_json::to_string(&ev).unwrap();
//! let de : events::Event = serde_json::from_str(&se).unwrap();
//!
//! assert_eq!(de.id, ev.id)
//! # }
//! ```
use effects;
use std::collections::VecDeque;

/// A struct representing an event that occurs in a quarter.
/// Events have a name, identifier and description.
/// When an event is `activate`d by a die roll, it
/// performs its effects.
#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub name: String,
    pub id: i32,
    pub desc: String,
    pub chance: i32,
    pub effects: Vec<effects::EventEffect>,
}

/// A ring buffer of events.
#[derive(Debug, Serialize, Deserialize)]
pub struct EventQueue {
    events: VecDeque<Event>,
}

impl EventQueue {
    /// Create an empty EventQueue with capacity cap.
    pub fn new(cap: usize) -> Self {
        EventQueue { events: VecDeque::with_capacity(cap) }
    }

    /// Push an Event onto the end of the queue. If it is full, pop the first
    /// element beforehand.
    pub fn push(&mut self, e: Event) {
        if self.is_full() {
            self.pop();
        }
        self.events.push_back(e)
    }

    /// Pop the first element off the queue.
    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop_front()
    }

    /// Check if the queue is full.
    fn is_full(&self) -> bool {
        self.events.capacity() == self.events.len()
    }
}

impl Event {

    /// Attempt to activate the event's effects with a provided roll.
    ///
    /// ## Arguments
    ///
    /// * `roll` - An integer that represents a randomly rolled number
    #[allow(dead_code)]
    pub fn activate(&self, roll: i32) {
        if roll <= self.chance {
            // TODO: perform event effects
        }
    }
}

