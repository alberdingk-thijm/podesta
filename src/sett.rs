//! The sett module contains the information governing the sett struct,
//! the basic object which represents the growing settlement.
use quarters;
use buildings;
use regions;
use people;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sett {
    pub name: String,
    pub age: i32,
    pub pop: i32,
    pub gold: f64,
    pub reg: Rc<regions::Region>,
    /// List of quarters in the settlement.
    pub qrtrs: Vec<Rc<RefCell<quarters::Quarter>>>,
    /// List of buildings in the settlement
    pub bldgs: Vec<Rc<RefCell<buildings::Building>>>,
    /// List of heroes in the settlement
    pub heroes: Vec<Rc<RefCell<people::Hero>>>,
    /// Population needed before a new quarter is added.
    pub nextqrtr: i32,
    /// Flag for if settlement is coastal.
    pub coastal: bool,
}

impl Sett {

    /// Create a new Settlement
    pub fn new(n: String,
               reg: Rc<regions::Region>,
               qt: quarters::QType,
               r: people::Race,
               coast: bool,
    ) -> Sett {
        // get the starting population based on the region's growth
        let pop : i32 = 50 * reg.growth;
        Sett {
            name: n,
            age: 0,
            pop: pop,
            gold: reg.starting_gold,
            reg: reg,
            qrtrs: vec![Rc::new(
                RefCell::new(quarters::Quarter::new("Main", qt, pop, r)))],
            bldgs: vec!(),
            // TODO: get a starting governor
            heroes: vec!(),
            nextqrtr: pop * 2,
            coastal: coast,
        }
    }

    /// Execute settlement timestep
    pub fn step(&mut self) {
        let mut newpop = 0;
        self.age += 1;
        // call each quarter's step
        for q in &self.qrtrs {
            q.borrow_mut().step(self.reg.growth);
            newpop += q.borrow().pop;
        }
        self.pop = newpop;
        // accumulate gold
        self.collect_gold();
    }

    /// Add quarter
    /// Gain a small amount of gold for doing so.
    /// Move a fraction of the sett's population to the new quarter,
    /// times the growth bonus.
    pub fn add_quarter(&mut self,
                       n: String,
                       qt: quarters::QType,
                       r: people::Race,
    ) -> Result<(), quarters::BuildError>
    {
        if qt == quarters::QType::Port && !self.coastal {
            return Err(quarters::BuildError::InlandPort);
        }
        let ref mut qrtrs = self.qrtrs;
        // make sure quarter is not already present
        if qrtrs.iter().any(|ref x| x.borrow().name == n) {
            return Err(quarters::BuildError::AlreadyExists);
        }
        // ensure pop is high enough
        if self.pop < self.nextqrtr {
            return Err(quarters::BuildError::NotEnoughPop);
        }
        // remove pop from existing quarters equally
        // TODO:
        // This method will hollow out older quarters unfairly,
        // leading to the main quarter risking becoming negative
        // while newer quarters lose a smaller % of their starting
        // population (since the population that moved there initially was
        // much larger, due to self.nextqrtr doubling).
        // This needs to be changed to fairly and naturally split
        // the population coming into the new quarter.
        let nqrtrs = qrtrs.len() as i32;
        for q in qrtrs.iter_mut() {
            let mut qb = q.borrow_mut();
            // add 1 because we are counting the new quarter
            qb.pop -= self.pop / (nqrtrs + 1);
        }
        let newpop = self.pop / (nqrtrs + 1);
        //TODO?: multiply number by growth bonus => newpop?
        qrtrs.push(Rc::new(
                RefCell::new(quarters::Quarter::new(&n, qt, newpop, r))));
        // receive gold bonus
        self.gold += 100.0;
        self.nextqrtr *= 2;
        Ok(())
    }

    #[allow(unused_variables)]
    /// Add a building
    pub fn add_building(&self, name: &str, q: Rc<RefCell<quarters::Quarter>>)
    -> Result<String, quarters::BuildError>{
        // Get the plans for the building
        // Check that self has enough gold to pay the cost
        unimplemented!()
    }

    /// Increment the total amount of gold in the settlement based on the
    /// state of its quarters.
    pub fn collect_gold(&mut self) {
        //TODO: placeholder incrementer
        self.gold += 1f64;
    }
}

pub trait HasName {
    fn get_name(&self) -> &str;
}

pub fn find_by_name<'a, 'b, T: HasName>(v: &'a [T], name: &'b str)
-> Option<&'a T> {
    v.iter().find(|&x| x.get_name() == name)
}

impl HasName for Sett {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Sett {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, located in {} {}.\n\
               steps: {} | pop: {} | gold: {}\n\
               Quarters:\n{}",
               self.name,
               if self.coastal { "coastal" } else { "inland" },
               self.reg, self.age, self.pop, self.gold,
               self.qrtrs.iter().map(|q| {
                   format!("- {}\n", *(q.borrow()))
               }).collect::<String>())
    }
}
