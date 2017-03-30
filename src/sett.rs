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
    pub nextqrtr: i32,
    pub flags: SettFlags,
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
            nextqrtr: pop * 2,
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
    pub fn add_quarter(&mut self,
                       n: String,
                       qt: quarters::QType,
                       r: people::Race,
    ) -> Result<String, quarters::BuildError>
    {
        // make sure quarter is not already present
        if let Some(_) = self.find_quarter(&n) {
            return Err(quarters::BuildError::AlreadyExists);
        }
        // ensure pop is high enough
        if self.pop < self.nextqrtr {
            return Err(quarters::BuildError::NotEnoughPop);
        }
        // remove pop from existing quarters equally
        let nqrtrs = self.qrtrs.len() as i32;
        for q in self.qrtrs.iter_mut() {
            q.pop -= self.nextqrtr / nqrtrs;
            //TODO: does this actually modify the struct or just a borrowed iterator?
            //TODO: fix using Rc or Box?
        }
        let newpop = self.nextqrtr;
        // multiply number by growth bonus => newpop?
        self.qrtrs.push(quarters::Quarter::new(&n, qt, newpop, r));
        // receive gold bonus
        self.gold += 100.0;
        self.nextqrtr *= 2;
        Ok("New quarter added!".to_string())
    }

    /// Find a quarter in the settlement based on its name.
    pub fn find_quarter(&self, name: &str) -> Option<&quarters::Quarter> {
        for q in &self.qrtrs {
            if q.name == name {
                return Some(q);
            }
        }
        None
    }
}
