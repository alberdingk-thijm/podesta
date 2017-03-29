//! The sett module contains the information governing the sett struct,
//! the basic object which represents the growing settlement.
use quarters;
use regions;
use people;

#[derive(Debug)]
pub struct Sett {
    pub name: String,
    pub age: i32,
    pub pop: i32,
    pub gold: f64,
    pub reg: regions::Region,
    pub qrtrs: Vec<quarters::Quarter>,
    pub flags: SettFlags,
    pub em: events::EventManager,
}

#[derive(Debug)]
pub enum SettFlags {
    Coastal,
    Inland,
}

impl Sett {
    /// Create a new Settlement
    pub fn new(n: &str,
               reg: regions::Region,
               qt: quarters::QType,
               r: people::Race,
               f: SettFlags,
    ) -> Sett {
        // get the starting population based on the region's growth
        let pop : i32 = 50 * reg.growth;
        Sett {
            name: n.to_string(),
            age: 0,
            pop: pop,
            gold: reg.starting_gold,
            reg: reg,
            qrtrs: vec!(quarters::Quarter::new("main", qt, pop, r)),
            flags: f,
        }
    }

    /// Execute settlement timestep
    pub fn step(&self) {
        // call each quarter's step
        // accumulate gold
        unimplemented!()
    }

    /// Add quarter
    /// Gain a small amount of gold for doing so.
    /// Move a fraction of the sett's population to the new quarter,
    /// times the growth bonus.
    pub fn add_quarter(&self,
                       n: String,
                       qt: quarters::QType,
                       r: people::Race,
    ) -> Result<Self, quarters::BuildErr>
    {
        // make sure quarter is not already present
        if let Some(_) = self.find_quarter(n) {
            return Err(quarters::BuildErr);
        }
        // ensure pop is high enough
        // remove pop from existing quarters equally
        // multiply number by growth bonus => newpop
        // call quarter::Quarter::new(n, qt, newpop, r);
        // receive gold bonus
        unimplemented!()
    }

    /// Find a quarter in the settlement based on its name.
    pub fn find_quarter(&self, name: String) -> Option<&quarters::Quarter> {
        for q in &self.qrtrs {
            if q.name == name {
                return Some(q);
            }
        }
        None
    }
}
