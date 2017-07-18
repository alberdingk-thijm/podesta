//! The sett module contains the information governing the sett struct,
//! the basic object which represents the growing settlement.
use quarters;
use buildings;
use regions;
use people;
use events;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sett {
    pub name: String,
    pub age: i32,
    pub pop: f64,  // use a float for more precise updating (display as an int)
    pub gold: f64,
    pub reg: Rc<regions::Region>,
    /// List of quarters in the settlement.
    pub qrtrs: Vec<Rc<RefCell<quarters::Quarter>>>,
    // List of buildings in the settlement
    //pub bldgs: Vec<Rc<RefCell<buildings::Building>>>,
    // List of heroes in the settlement
    //pub heroes: Vec<Rc<RefCell<people::Hero>>>,
    /// Turns before a new quarter is added.
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
        let pop : f64 = 50.0 * reg.growth;
        Sett {
            name: n,
            age: 0,
            pop: pop,
            gold: reg.starting_gold,
            reg: reg,
            qrtrs: vec![Rc::new(
                RefCell::new(quarters::Quarter::new("Main", qt, pop, r)))],
            //bldgs: vec!(),
            // TODO: get a starting governor
            //heroes: vec!(),
            nextqrtr: 50,
            coastal: coast,
        }
    }

    /// Execute settlement timestep and perform the following actions:
    /// - Increment settlement age by 1.
    /// - Perform quarters::step() for each quarter, and
    /// - calculate new population from the quarters.
    /// - Increment gold for the settlement.
    /// - Compute an event map for the step and return it.
    pub fn step(&mut self) -> events::EventMap {
        // new population (sum of quarters' population)
        let mut newpop = 0f64;
        self.age += 1;
        // call each quarter's step
        for q in &self.qrtrs {
            q.borrow_mut().step(self.reg.growth as f64);
            newpop += q.borrow().pop;
        }
        self.pop = newpop;
        // accumulate gold
        self.collect_gold();
        // compute event chances and return an eventmap
        self.compute_events()
    }

    /// Compute the event chances for this step.
    fn compute_events(&self) -> events::EventMap {
        let mut map = events::EventMap::new(self.age);
        // get all buildings' events
        /*
        let eventcs = {
            let qrtrs = self.qrtrs.iter().map(|q| q.borrow());
            let bldgs = qrtrs.flat_map(|q| q.bldgs.iter().map(|b| b.borrow()));
            bldgs.flat_map(|b| b.get_eventchances().iter())
        };
        */
        for q in &self.qrtrs {
            for b in &q.borrow().bldgs {
                // get all EventChances
                let eventcs = {
                    let bldg = b.borrow();
                    bldg.get_events()
                };
                map.add_chances(eventcs);
            }
        }
        map
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
        // make sure quarter with same unique name is not already present
        if qrtrs.iter().any(|ref x| x.borrow().name == n) {
            return Err(quarters::BuildError::AlreadyExists);
        }
        // TODO: add logic for when new quarter should be added
        let newpop = 50.0 * self.reg.growth;
        qrtrs.push(Rc::new(
                RefCell::new(quarters::Quarter::new(&n, qt, newpop, r))));
        // receive gold bonus
        self.gold += 100.0;
        self.nextqrtr *= 2;
        self.pop += newpop;
        Ok(())
    }

    #[allow(unused_variables)]
    /// Add a building
    pub fn add_building(&mut self,
                        plan: Rc<buildings::BuildingPlan>,
                        q: Rc<RefCell<quarters::Quarter>>)
    -> Result<(), quarters::BuildError> {
        // Check that self has enough gold to pay the cost
        if self.gold < plan.cost {
            return Err(quarters::BuildError::NotEnoughGold);
        }

        self.gold -= plan.cost;
        q.borrow_mut().add_building(plan)
    }

    /// Increment the total amount of gold in the settlement based on the
    /// state of its quarters.
    pub fn collect_gold(&mut self) {
        //TODO: placeholder incrementer
        self.gold += 0.01f64 * self.pop as f64;
        for q in &self.qrtrs {
            self.gold += q.borrow_mut().collect_gold();
        }
        //self.gold += 0.04f64 * self.heroes.len() as f64;
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
               self.reg, self.age, self.pop as i64, self.gold as i64,
               self.qrtrs.iter().map(|q| {
                   format!("- {}\n", *(q.borrow()))
               }).collect::<String>())
    }
}
