//! ## Example
//!
//! ```
//! use podesta::events;
//! let ev = events::Event {
//!     name: "Fire".to_string(),
//!     id: 1,
//!     desc: "%s has erupted into flames!".to_string(),
//!     chance: 3,
//!     effects: vec!("Kill_2".to_string(), "Dam_2".to_string()),
//! };
//! ```

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
    Dam(i32, i32),
    Riot(i32, i32, i32),
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

