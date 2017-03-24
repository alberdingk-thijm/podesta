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
//!     effects: vec!("Kill_2".to_string(), "Dam_2".to_string()),
//! };
//! let se = serde_json::to_string(&ev).unwrap();
//! let de : events::Event = serde_json::from_str(&se).unwrap();
//!
//! assert_eq!(de.id, ev.id)
//! # }
//! ```
use std::str;

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
    #[serde(rename = "event")]
    pub effects: Vec<String>,
    // TODO: convert to Vec<event effect>
}

pub enum EventEffect {
    Kill(i32, i32),
    Damage(i32, i32),
    Riot(i32, i32, i32),
    Grow(i32, i32),
    Build(i32, i32),
    Gold(i32, i32),
    Hero { mage: i32, bard: i32, merch: i32, crime: i32, rlg: i32, war: i32 },
    Item(i32, i32, i32),
}

impl str::FromStr for EventEffect {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // first term is eventeffect name,
        // second is which value to use
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

