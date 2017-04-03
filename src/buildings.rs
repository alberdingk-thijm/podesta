use std::default;
use quarters;
//use people;
use sett;

#[derive(Debug)]
pub struct Building {
    pub name: String,
    pub id: i32,
    pub loc: Box<quarters::Quarter>,
    pub btype: quarters::QType,
    pub events: Vec<EventChance>,
    pub bspeed: f64,
    pub condition: BldgCond,
    //pub occupants: Vec<people::Hero>,
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


#[derive(Serialize, Deserialize, Debug)]
pub struct EventChance {
    name: String,
    chance: f32,
}

impl Building {

    pub fn new(plans: BuildingPlan, loc: Box<quarters::Quarter>) -> Building {
        Building {
            name: plans.name,
            id: plans.id,
            loc: loc,
            btype: plans.btype,
            events: plans.events,
            bspeed: plans.build,
            condition: BldgCond::InProgress,
        }
    }
}

impl sett::HasName for Building {
    fn get_name(&self) -> &str {
        &self.name
    }
}
