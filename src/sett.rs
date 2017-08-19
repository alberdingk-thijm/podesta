//! The sett module contains the information governing the sett struct,
//! the basic object which represents the growing settlement.
use quarters;
use buildings;
use regions;
use people;
use events;
use effects;
use prompts;
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
    /// Turns before a new quarter is added.
    pub nextqrtr: i32,
    /// Flag for if settlement is coastal.
    pub coastal: bool,
    //flag for growth/build/gold
    pub boosts: effects::EffectFlags,
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
        let pop = 50.0 * reg.growth;
        Sett {
            name: n,
            age: 0,
            pop: pop,
            gold: reg.starting_gold,
            reg: reg,
            qrtrs: vec![Rc::new(
                RefCell::new(quarters::Quarter::new("Main", qt, pop, r)))],
            // TODO: get a starting governor and governing hall?
            nextqrtr: 50,
            coastal: coast,
            boosts: effects::EffectFlags::default(),
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
            q.borrow_mut().step(self.reg.growth);
            newpop += q.borrow().pop;
        }
        // include the growth bonus based on diff in pop
        self.pop = newpop + self.boosts.grow.next().unwrap_or(1.0) * (newpop - self.pop).abs();
        // accumulate gold (TODO: replace None with boost option)
        self.collect_gold();
        // compute event chances and return an eventmap
        self.compute_events()
    }

    /// Compute the event chances for this step.
    fn compute_events(&self) -> events::EventMap {
        let mut map = events::EventMap::new(self.age);
        // get all buildings' events
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
        // Check that prerequisites are built
        if let Some(ref preqs) = plan.preq {
            for p in preqs.iter() {
                let bnames = {
                    let mut v = vec![];
                    for q in self.qrtrs.iter() {
                        for b in q.borrow().bldgs.iter() {
                            v.push(b.borrow().name.clone())
                        }
                    }
                    v
                };
                if bnames.iter().find(|b| b == &p).is_none() {
                    return Err(quarters::BuildError::PrereqsMissing);
                }
            }
        }

        self.gold -= plan.cost;
        q.borrow_mut().add_building(plan)
    }

    /// Increment the total amount of gold in the settlement based on the
    /// state of its quarters. For each member of the population, collect
    /// 0.01 gold times the optional boost.
    pub fn collect_gold(&mut self) {
        //TODO: placeholder incrementer
        let boost = self.boosts.gold.next().unwrap_or(1.0);
        self.gold += 0.01f64 * boost * self.pop;
        for q in &self.qrtrs {
            self.gold += q.borrow_mut().collect_gold() * boost;
        }
        self.gold += self.boosts.gold_bonus.next().unwrap_or(0.0);
    }

    /// Return a wrapped Quarter if one by the given name can be found.
    pub fn find_quarter(&self, name: &str) -> Option<Rc<RefCell<quarters::Quarter>>> {
        self.qrtrs.iter().find(|&q| q.borrow().name == name).map(|q| q.clone())
    }

    /// Return a wrapped Building if one by the given name in the given quarter can be found.
    pub fn find_building(&self, bname: &str, qname: &str)
        -> Option<Rc<RefCell<buildings::Building>>>
    {
        self.find_quarter(qname).and_then(|q| q.borrow().find_building(bname))
    }

    /// Return (quarter name, wrapped Building) pairs if any buildings by
    /// the given name can be found.
    pub fn find_buildings(&self, bname: &str)
        -> Vec<(String, Rc<RefCell<buildings::Building>>)>
    {
        use prompts::Described;
        let qrtrs = self.qrtrs.iter();
        qrtrs.map(|q| (q.borrow().name(),
                            q.borrow().find_building(bname)))
            // we now have (String, Option<...> pairs)
            // so map to Option<(String, ...)> and filter
             .filter_map(|(q, ob)| match ob {
                 Some(b) => Some((q, b)),
                 None => None,
             }).collect::<Vec<_>>()
    }

    /// Return a wrapped Hero if one by the given name can be found.
    pub fn find_hero(&self, bname: &str, qname: &str, hname: &str)
        -> Option<Rc<RefCell<people::Hero>>>
    {
        self.find_building(bname, qname)
            .and_then(|b| b.borrow().find_hero(hname))
    }

    pub fn find_heroes(&self, hname: &str)
        -> Vec<(String, String, Rc<RefCell<people::Hero>>)>
    {
        // get all buildings
        // map (qname, b) to (qname, bname, h)
        unimplemented!()
    }
}

impl prompts::Described for Sett {
    fn name(&self) -> String {
        self.name.clone()
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
