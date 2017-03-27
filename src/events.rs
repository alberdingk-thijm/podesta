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
    pub effects: Vec<String>,
    // TODO: convert to Vec<event effect>
}

pub enum EventEffect {
    Kill { dead: Roll, viralpt: Option<i32>, area: Area },
    Damage { crumbled: Roll, viralpt: Option<i32>, area: Area },
    Riot { steps: Roll, prod: f64, area: Area },
    Grow { bonus: Roll, area: Area },
    Build { bonus: Roll, area: Area },
    Gold { value: Roll, bonus: f64, steps: Roll },
    Hero { level: Roll, classes: Vec<people::Class> },
    Item { value: Roll, magical: f64 },
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

