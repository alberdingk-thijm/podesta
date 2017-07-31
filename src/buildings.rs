use std::default;
use std::fmt;
use quarters;
use people;
use sett;
use effects;
use items;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use rand::{self, Rng};

#[derive(Debug, Serialize, Deserialize)]
pub struct Building {
    pub name: String,
    pub plan: Rc<BuildingPlan>,
    pub cond: BldgCond,
    pub occupants: Vec<Rc<RefCell<people::Hero>>>,
    pub items: Vec<Rc<RefCell<items::Item>>>,
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

#[derive(Debug)]
/// Errors specific to buildings and their use.
pub enum OccupyError {
    /// Building's condition prevents its use.
    NotInUse,
    /// Occupant not valid in building.
    InvalidOccupant,
    /// Item not valid in building.
    InvalidItem,
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
            items: vec!(),
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
                    // remove all occupants & items when building becomes ruined
                    // TODO: should this be handled by the manager?
                    // TODO: manager can alert that the building is ruined
                    // TODO: and that the occupants have left and items destroyed
                    self.occupants.clear();
                    self.items.clear();
                    BldgCond::Ruined
                } else {
                    BldgCond::InUse((n - 1.0 + self.boosts.build_bonus.next()
                                     .unwrap_or(0.0)).min(100.0))
                }
            },
            BldgCond::Ruined => BldgCond::Ruined,
        };
        //TODO: kill heroes if building has growth subtraction
        //TODO: destroy items if building has build subtraction
        let mut r = rand::thread_rng();
        for hero in self.occupants.iter() {
            hero.borrow_mut().step(r.gen_range(1, 101));
        }
        for item in self.items.iter() {
            item.borrow_mut().step();
        }
    }

    /// Add occupant to building.
    /// Return an Error if the building cannot accept the occupant.
    pub fn add_occupant(&mut self, hero: Rc<RefCell<people::Hero>>) -> Result<(), OccupyError> {
        match self.cond {
            BldgCond::InUse(_) => {
                self.occupants.push(hero);
                Ok(())
                //TODO: only allow buildings specified by hero's class
                //TODO: would require changes to manager::rand_building()
                /*
                if hero.borrow().class.bldgs.contains(&self.name) {
                    self.occupants.push(hero);
                    Ok(())
                } else {
                    Err(OccupyError::InvalidOccupant)
                }
                */
            },
            _ => Err(OccupyError::NotInUse),
        }
    }

    /// Add item to building.
    /// Return an Error if the building cannot accept the item.
    pub fn add_item(&mut self, item: Rc<RefCell<items::Item>>) -> Result<(), OccupyError> {
        match self.cond {
            BldgCond::InUse(_) => {
                //TODO: add element to building plan specifying
                //TODO: kinds of items building can hold
                self.items.push(item);
                Ok(())
            },
            _ => Err(OccupyError::NotInUse),
        }
    }

    /// Collect gold.
    /// For each occupant of the building, collect an extra 0.04 gold.
    /// For each item in the building, collect an extra 0-0.0x gold, where
    /// x is the item's power.
    pub fn collect_gold(&mut self) -> f64 {
        let boost : f64 = self.boosts.gold.next().unwrap_or(1.0);
        (self.occupants.len() as f64 * 0.04f64
         + self.items.iter().fold(0.0, |acc, x| acc + x.borrow().collect_gold() * boost))
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
