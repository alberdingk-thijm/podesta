/// The manager for a settlement
/// Tracks metavariables and manages the event queue and its effects on the
/// settlement's members.

use parser;
use sett;
use history;
use events;

#[derive(Debug, Serialize, Deserialize)]
pub struct Manager {
    datafiles: parser::DataFiles,
    sett: Option<sett::Sett>,
    hist: history::History,
    queue: events::EventQueue,
}

impl Manager {
    /// Create a new Manager with the given data files.
    /// Note that until build_sett() is called, no settlement actually exists.
    pub fn new() -> Self {
        unimplemented!()
    }

    /// Initialize a new settlement and store it in the manager.
    pub fn build_sett(&mut self) {
        unimplemented!()
    }

    /// Initialize a new quarter and store it in the manager's sett.
    pub fn build_quarter(&mut self) {
        unimplemented!()
    }

    /// Initialize a new building and store it in the manager's sett's quarter.
    pub fn build_building(&mut self) {
        unimplemented!()
    }

    /// Pop an event and perform its effects on the sett.
    pub fn activate_event(&mut self) {
        unimplemented!()
    }
}
