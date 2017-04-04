//! The sett module contains the information governing the sett struct,
//! the basic object which represents the growing settlement.
use quarters;
use buildings;
use regions;
use people;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Sett {
    pub name: String,
    pub age: i32,
    pub pop: i32,
    pub gold: f64,
    pub reg: regions::Region,
    /// List of quarters in the settlement.
    pub qrtrs: RefCell<Vec<quarters::Quarter>>,
    /// List of buildings in the settlement
    pub bldgs: RefCell<Vec<buildings::Building>>,
    /// List of heroes in the settlement
    pub heroes: RefCell<Vec<people::Hero>>,
    /// Population needed before a new quarter is added.
    pub nextqrtr: i32,
    /// Flags for settlement info.
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
            qrtrs: RefCell::new(vec![
                                quarters::Quarter::new("main",
                                                       qt,
                                                       pop,
                                                       r)]
                                ),
            bldgs: RefCell::new(vec!()),
            // TODO: get a starting governor
            heroes: RefCell::new(vec!()),
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
        let mut qrtrs = self.qrtrs.borrow_mut();
        // make sure quarter is not already present
        if qrtrs.iter().any(|ref x| x.name == n) {
        //if let Some(_) = find_by_name(&qrtrs, &n) {
            return Err(quarters::BuildError::AlreadyExists);
        }
        // ensure pop is high enough
        if self.pop < self.nextqrtr {
            return Err(quarters::BuildError::NotEnoughPop);
        }
        // remove pop from existing quarters equally
        let nqrtrs = qrtrs.len() as i32;
        for q in qrtrs.iter_mut() {
            // add 1 because we are counting the new quarter
            q.pop -= self.nextqrtr / (nqrtrs + 1);
        }
        let newpop = self.nextqrtr / nqrtrs;
        // multiply number by growth bonus => newpop?
        qrtrs.push(quarters::Quarter::new(&n, qt, newpop, r));
        // receive gold bonus
        self.gold += 100.0;
        self.nextqrtr *= 2;
        Ok("New quarter added!".to_string())
    }

    /// Add a building
    pub fn add_building(&self, name: &str, q: Box<quarters::Quarter>)
    -> Result<String, quarters::BuildError>{
        // Get the plans for the building
        // Check that self has enough gold to pay the cost
        unimplemented!()
    }

    pub fn add_pop(&mut self, num: i32) {
        self.pop += num;
    }
}

pub trait HasName {
    fn get_name(&self) -> &str;
}

pub fn find_by_name<'a, 'b, T: HasName>(v: &'a [T], name: &'b str)
-> Option<&'a T> {
    v.iter().find(|&x| x.get_name() == name)
}

macro_rules! find_named {
    ($a:expr, $name:ident) => ($a.iter().find(|&x| x.name == $name))
}
