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
        if qt == quarters::QType::Port && !self.coastal {
            return Err(quarters::BuildError::InlandPort);
        }
        let ref mut qrtrs = self.qrtrs; //borrow_mut();
        // make sure quarter is not already present
        if qrtrs.iter().any(|ref x| x.borrow().name == n) {
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
            let mut qb = q.borrow_mut();
            // add 1 because we are counting the new quarter
            qb.pop -= self.nextqrtr / (nqrtrs + 1);
        }
        let newpop = self.nextqrtr / nqrtrs;
        // multiply number by growth bonus => newpop?
        qrtrs.push(Rc::new(
                RefCell::new(quarters::Quarter::new(&n, qt, newpop, r))));
        // receive gold bonus
        self.gold += 100.0;
        self.nextqrtr *= 2;
        Ok("New quarter added!".to_string())
    }

    #[allow(unused_variables)]
    /// Add a building
    pub fn add_building(&self, name: &str, q: Rc<RefCell<quarters::Quarter>>)
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

impl HasName for Sett {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Sett {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, located in {} {}. {} steps old. Population {}.\n\
               Quarters:\n{}",
               self.name,
               if self.coastal { "coastal" } else { "inland" },
               self.reg, self.age, self.pop,
               self.qrtrs.iter().map(|q| {
                   format!("- {}\n", *(q.borrow()))
               }).collect::<String>())
    }
}
