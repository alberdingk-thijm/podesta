use std::default;
use std::fmt;
use quarters;
use people;
use sett;
use effects;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Building {
    pub name: String,
    pub plan: Rc<BuildingPlan>,
    pub cond: BldgCond,
    pub occupants: Vec<people::Hero>,
    pub boosts: effects::EffectFlags,
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
    pub events: HashMap<String, f64>,
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
    /// Building is not yet complete (no bonuses); any damage resets to 0.0.
    InProgress(f64),
    /// Building bonuses have full effects. Each timestep, reduces by 1%.
    /// TODO: Accident chance also increases slightly with age.
    /// Goes from 100.0 to 0.0.
    InUse(f64),
    /// Building too destroyed to use.
    Ruined,
}

impl fmt::Display for BldgCond {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            BldgCond::InProgress(n) => format!("{:.1}% complete", n),
            BldgCond::InUse(n) => format!("{:.1}% durability", n),
            BldgCond::Ruined => format!("in ruins"),
        })
    }
}

impl default::Default for BldgCond {
    fn default() -> BldgCond { BldgCond::InProgress(0.0) }
}

impl Building {
    /// Create a new building object.
    pub fn new(plan: Rc<BuildingPlan>) -> Building {
        Building {
            name: plan.name.clone(),
            plan: plan,
            cond: BldgCond::InProgress(0.0),
            occupants: vec!(),
            boosts: effects::EffectFlags::default(),
        }
    }

    /// Execute a timestep for the building, aging it (or progressing in its
    /// construction).
    /// Add the given bonus to the building (note: bonus can be negative).
    pub fn step(&mut self) {
        self.cond = match self.cond {
            BldgCond::InProgress(n) => {
                if n >= 100.0 {
                    BldgCond::InUse(100.0)
                } else {
                    BldgCond::InProgress(n + (self.plan.build
                                              * self.boosts.build.next().unwrap_or(1.0))
                                         + self.boosts.build_bonus.next().unwrap_or(0.0))
                }
            },
            BldgCond::InUse(n) => {
                if n <= 0.0 {
                    // remove all occupants when building becomes ruined
                    // TODO: should this be handled by the manager?
                    // TODO: manager can alert that the building is ruined
                    // TODO: and that the occupants have left
                    self.occupants.clear();
                    BldgCond::Ruined
                } else {
                    BldgCond::InUse((n - 1.0 + self.boosts.build_bonus.next().unwrap_or(0.0)).min(100.0))
                }
            },
            BldgCond::Ruined => BldgCond::Ruined,
        }
    }

    /// Get a new map of event chances for each event possible at the building
    /// based on the building's condition.
    pub fn get_events(&self) -> HashMap<String, f64> {
        let mut events = self.plan.events.clone();
        events.iter_mut().map(|(nm, ch)| {
            (nm.clone(), *ch *
             match self.cond {
                 // when in use, return the parameter divided by 100.0
                 BldgCond::InUse(n) => n / 100.0,
                 // otherwise, event has no chance of occurring
                 _ => 0.0,
             })
        }).collect::<HashMap<_,_>>()
    }
}

impl sett::HasName for Building {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Building {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {} - {} - hosts {} people",
               self.name,
               self.plan.btype,
               self.cond,
               self.occupants.len())
   }
}
