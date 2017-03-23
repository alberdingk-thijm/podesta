//!
//!
//!

/// A struct representing an event that occurs in a quarter.
/// Events have a name, identifier and description.
/// When an event is ```activate```d by a die roll, it
/// performs its effects.
///
/// # Examples
///
/// The events available are stored in lib/data/events.json,
/// and hence can be serialized and deserialized.
///
/// ```
/// use events::Event;
/// use serde_json;
/// let ev = Event {
///     name: "Fire".to_string(),
///     id: 1,
///     desc: "%s has erupted into flames!".to_string(),
///     chance: 3,
///     effects: vec!("Kill_2".to_string(), "Dam_2".to_string()),
/// };
///
/// let serialized = serde_json::to_string(&ev).unwrap();
/// let deserialized: Event = serde_json::from_str(&serialized).unwrap();
///
/// assert_eq!(ev, deserialized)
/// ```
///
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

impl Event {

    /// Attempt to activate the event with a provided roll.
    /// If it falls beneath the chance threshold, perform the
    /// event effects specified by self.event.
    #[allow(dead_code)]
    pub fn activate(&self, roll: i32) {
        if roll <= self.chance {
            // TODO: perform event effects
        }
    }
}
