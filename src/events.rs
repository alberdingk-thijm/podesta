
#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub name: String,
    pub id: i32,
    pub desc: String,
    pub chance: i32,
    pub event: Vec<String>,
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
