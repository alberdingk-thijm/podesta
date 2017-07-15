use std::default;
use quarters;
use people;
use sett;
use std::rc::Rc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Building {
    pub name: String,
    //pub id: i32,
    pub btype: quarters::QType,
    pub events: Vec<EventChance>,
    pub bspeed: f64,
    pub condition: BldgCond,
    pub occupants: Vec<people::Hero>,
}

#[derive(Serialize, Deserialize, Debug)]
/// Plan for a building.
/// Generated from lib/data/buildings.json
pub struct BuildingPlan {
    pub name: String,
    pub id: i32,
    #[serde(rename = "type")]
    pub btype: quarters::QType,
    pub preq: Option<Vec<String>>,
    pub cost: f64,
    pub build: f64,
    pub events: Vec<EventChance>,
}

impl BuildingPlan {
    pub fn draft_building(&self) -> Building {
        //FIXME: events needs to be copied or moved correctly
        Building::new(&self.name, self.btype, self.build, self.events.clone())
    }
}

/// Enum representing condition of a building.
/// # Improvements
/// * Add ability to perform repairs for gold.
/// * Pay gold to remove ruined buildings.
#[derive(Serialize, Deserialize, Debug)]
pub enum BldgCond {
    /// Building is not yet complete (no bonuses); any damage resets to -100.
    InProgress = -100,
    /// Building bonuses have full effects. Each timestep, reduces by 1%.
    /// Accident chance also increases slightly with age.
    New = 0,
    /// Building too destroyed to use.
    Ruined = 100,
}

impl default::Default for BldgCond {
    fn default() -> BldgCond { BldgCond::InProgress }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventChance {
    name: String,
    chance: f32,
}

impl Building {
    /// Create a new building object.
    pub fn new(n: &str,
               btype: quarters::QType,
               speed: f64,
               events: Vec<EventChance>
               ) -> Building
    {
        Building {
            name: n.to_string(),
            // DEPRECATED:
            //id: plans.id,
            btype: btype,
            events: events,
            bspeed: speed,
            condition: BldgCond::InProgress,
            occupants: vec!(),
        }
    }

    /// Execute a timestep for the building, aging it (or progressing in its
    /// construction).
    pub fn step(&mut self) {
        // add bspeed if still in progress
        // set bspeed to 1 once built
        // if condition reaches 0, add events to tracker
        // if condition reaches 100, remove events from tracker
    }
}

impl sett::HasName for Building {
    fn get_name(&self) -> &str {
        &self.name
    }
}
