use std::default;
use std::fmt;
use quarters;
use people;
use sett;
use std::rc::Rc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Building {
    pub name: String,
    //pub id: i32,
    pub plan: Rc<BuildingPlan>,
    //pub btype: quarters::QType,
    //pub events: Vec<EventChance>,
    //pub bspeed: f64,
    pub condition: f64, //BldgCond,
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

impl fmt::Display for BuildingPlan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}): pre-reqs {}, costs {}", 
               self.name,
               self.btype,
               match self.preq {
                   Some(ref preqs) => (&preqs).join(" & "),
                   None => String::from("n/a"),
               },
               self.cost)
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
    pub fn new(plan: Rc<BuildingPlan>) -> Building {
        Building {
            name: plan.name.clone(),// to_string(),
            // DEPRECATED:
            //id: plans.id,
            plan: plan,
            //btype: btype,
            //events: events,
            //bspeed: speed,
            condition: -100.0, //BldgCond::InProgress,
            occupants: vec!(),
        }
    }

    /// Execute a timestep for the building, aging it (or progressing in its
    /// construction).
    pub fn step(&mut self) {
        //TODO: allow second parameter with possible repairs/damage to
        // add to condition?
        let age_rate = if self.condition < 0.0 {
            // add build rate if still in progress
            self.plan.build
        } else {
            // add 1 each step once built
            1.0
        };
        //TODO: building can become slightly "overage" on reaching 0
        //TODO: possible solution: in_progress boolean
        self.condition += age_rate;
        //TODO:
        // if condition reaches 0 for first time, add events to tracker
        // if condition reaches 100 for first time, remove events from tracker
        //TODO:
        // possible second boolean to show in_use (events can trigger)
    }
}

impl sett::HasName for Building {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Building {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}): {:.1}% {}, hosts {} people", 
               self.name,
               self.plan.btype,
               100.0 - self.condition.abs(),  // distance from new & complete
               if self.condition < 0.0 { "completion" } else { "durability" },
               self.occupants.len())
   }
}
