//! The manager for a settlement
//! Tracks metavariables and manages the event queue and its effects on the
//! settlement's members.

use libdata;
use time;
use sett;
use quarters;
use buildings;
use items;
use people;
use history;
use events;
use effects;
use prompts;
use std::fmt;
use std::error;
use std::rc::Rc;
use std::cell::RefCell;
use rand::{self, Rng};
use std::result;
//use std::io::{self, Read, Write};

macro_rules! choose_info {
    ($printexpr:expr, $auto:expr, $name:expr) => {
        print!("Choos{} ", if $auto { "ing" } else { "e" });
        println!($printexpr, $name);
    };
}

macro_rules! print_opt {
    ($dev:expr, $opt:expr) => {
        match $opt {
            Some(ref x) => dev_print!($dev, x),
            None => (),
        }
    };
}

macro_rules! enum_match {
    ($e:expr, $p: pat) => {
        match $e {
            $p => true,
            _ => false,
        }
    };
}

macro_rules! dev_print {
    ($dev:expr, $e:expr) => {
        if $dev {
            println!("{:?}", $e)
        } else {
            println!("{}", $e)
        }
    };
}

//TODO: code-condensing pattern for activate_event's matching on roll
/*
macro_rules! match_roll {
}
*/

type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Lib(libdata::LibError),
    Build(quarters::BuildError),
    NoSett,
    History,
    Event,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Lib(ref e) => e.fmt(f),
            Error::Build(ref e) => e.fmt(f),
            Error::NoSett => write!(f, "No sett found (first run 'new' or 'load')"),
            Error::History => write!(f, "Failed to update history log"),
            Error::Event => write!(f, "Failed to perform event"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Lib(ref err) => err.description(),
            Error::Build(ref err) => err.description(),
            Error::NoSett => "no sett found",
            Error::History => "unable to write history",
            Error::Event => "unable to perform event",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Lib(ref err) => Some(err),
            Error::Build(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<quarters::BuildError> for Error {
    fn from(err: quarters::BuildError) -> Error { Error::Build(err) }
}

impl From<libdata::LibError> for Error {
    fn from(err: libdata::LibError) -> Error { Error::Lib(err) }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manager {
    /// Game data files (stored in lib/data)
    datafiles: libdata::DataFiles,
    /// Game name files (stored in lib/names)
    namefiles: libdata::NameFiles,
    /// The game settlement
    sett: Option<sett::Sett>,
    /// The history tracker for game events
    hist: history::History,
    /// The queue of events in the settlement
    queue: events::EventQueue,
    /// Whether or not to make random choices automatically
    automate: bool,
    /// Whether or not to print additional information
    verbose: bool,
    /// Whether or not we are in dev mode
    dev: bool,
    /// The name of the game's save file
    savefile: String,
    /*
    /// Standard input
    stdin: R,
    /// Standard output
    stdout: W,
    */
}

impl Manager {
    /// Create a new Manager with the given data files.
    /// Note that until build_sett() is called, no settlement actually exists.
    pub fn new(pl: &libdata::PathList, verb: bool) -> Self {
        Manager {
            datafiles: libdata::DataFiles::from_pathlist(pl),
            namefiles: libdata::NameFiles::from_pathlist(pl),
            sett: None,
            hist: history::History::new(),
            queue: events::EventQueue::new(32),
            automate: false,
            verbose: verb,
            dev: false,
            savefile: format!("pod-{}.rbs", time::now().ctime()),
        }
    }

    /// Load a Manager from a file with the given name; if None is given,
    /// prompt the user for one.
    pub fn load(file: Option<String>) -> Result<Self> {
        match file {
            Some(filename) => Ok(filename),
            None => prompts::name_file(" to load: "),
        }.map_err(libdata::LibError::Prompt)
            .and_then(|f| libdata::load_rbs(f.as_str()))
            .map_err(Error::Lib)

    }

    /// Save a Manager to its save file, optionally prompting the user for a
    /// file name or accepting one.
    pub fn save(&self, file: Option<String>) -> Result<()> {
        // Use the given file, OR prompt for a file, OR use the default
        let savef = file.unwrap_or_else(||
            prompts::name_file(&format!(" to save to (default: {}): ",
                                        self.savefile))
                               .unwrap_or(self.savefile.clone()));
        libdata::save_rbs(self, &savef)
            .map_err(Error::Lib)
    }

    /// Toggle automation of build functions.
    pub fn toggle_auto(&mut self) {
        self.automate = !self.automate;
        if self.verbose { println!("Automation set to {}", self.automate) }
    }

    /// Toggle dev mode. TODO: Make not work in release.
    pub fn toggle_dev(&mut self) {
        self.dev = !self.dev;
        if self.verbose { println!("Dev mode set to {}", self.dev) }
    }

    /// Initialize a new settlement and store it in the manager.
    pub fn build_sett(&mut self, name_input: Option<String>, force: bool) {
        if self.sett.is_none() || force || Self::confirm_overwrite() {
            let nprompts = if self.automate { 0 } else { 2 };
            // Name (prompt for one if name_input is None)
            let name = match name_input {
                Some(n) => n,
                None => prompts::name_loop(1),
            };
            // Region
            choose_info!("{}'s region...", self.automate, name);
            let regchoice = prompts::choose_or_rand(&(self.datafiles.regions),
                                                    nprompts);
            let reg = self.datafiles.regions[regchoice].clone();
            // Coastal
            choose_info!("if {} is coastal...", self.automate, name);
            let coastchoice = prompts::bool_choose_or_rand(
                &format!("Is {} coastal? (y/n): ", name),
                &["y", "yes"], &["n", "no"], nprompts);
            // Quarter type
            choose_info!("the focus of {}'s main quarter...",
                         self.automate, name);
            let qchoice = prompts::choose_or_rand(
                &quarters::QType::get_qtype_names(coastchoice),
                nprompts);
            let qtype = quarters::QType::get_qtypes(coastchoice)[qchoice];
            // Race
            choose_info!("the majority race of {}'s main quarter...",
                         self.automate, name);
            let racenames = people::Race::iter_variant_names();
            let racechoice = prompts::choose_or_rand(
                &(racenames.collect::<Vec<_>>()), nprompts);
            let race = people::Race::iter_variants().nth(racechoice).unwrap();

            self.savefile = format!("{}.rbs", &name);

            self.sett = Some(
                sett::Sett::new(name, reg, qtype, race, coastchoice)
            );

            if self.verbose { println!("{}", self.sett.as_ref().unwrap()) }
        }
    }

    /// Return the name of the savefile.
    pub fn get_savefile(&self) -> String {
        self.savefile.clone()
    }

    /// Initialize a new quarter and store it in the manager's sett.
    pub fn build_quarter(&mut self, name_input: Option<String>) {
        match self.sett {
            Some(ref mut s) => {
                let nprompts = if self.automate { 0 } else { 2 };
                // Name (prompt for one if name_input is None)
                let name = match name_input {
                    Some(n) => n,
                    None => prompts::name_loop(1),
                };
                // Quarter type
                choose_info!("the focus of the {} quarter...",
                             self.automate, name);
                let qchoice = prompts::choose_or_rand(
                    &quarters::QType::get_qtype_names(s.coastal),
                    nprompts);
                let qtype = quarters::QType::get_qtypes(s.coastal)[qchoice];
                // Race
                choose_info!("the majority race of the {} quarter...",
                             self.automate, name);
                let racenames = people::Race::iter_variant_names();
                let racechoice = prompts::choose_or_rand(
                    &(racenames.collect::<Vec<_>>()), nprompts);
                let race = people::Race::iter_variants()
                    .nth(racechoice).unwrap();
                s.add_quarter(name.clone(), qtype, race).map_err(Error::Build)
            },
            None => Err(Error::NoSett),
        }.unwrap_or_else(|e| println!("Failed to construct quarter: {}", e))
    }

    /// Initialize a new building and store it in the manager's sett's quarter.
    #[allow(unused_variables)]
    pub fn build_building(&mut self,
                          name_input: Option<String>,
                          quarter_input: Option<String>)
    {
        match self.sett {
            Some(ref mut s) => {
                let ref plans = self.datafiles.plans;
                let mut plannames = plans.iter().map(|p| p.to_string());
                // Find index of plan to add
                let plan = match name_input {
                    Some(s) => plannames.position(|x| {
                        // check that name matches up to first space
                        x.split_whitespace().next()
                            .map(|sx| sx == s).unwrap_or(false)
                    }).ok_or(quarters::BuildError::NoPlanFound),
                    None => { prompts::choose(&plannames.collect::<Vec<_>>())
                        .map_err(|e| { quarters::BuildError::NoPlanFound })
                    },
                }.map(|i| plans[i].clone()).map_err(Error::Build);

                plan.and_then(|p| {
                    // Determine which quarter should contain the planned building
                    let qrtr = {
                        // Borrow s and plan within block to avoid borrow errors
                        let valid_qrtrs = s.qrtrs.iter().filter(|ref q| {
                            p.btype == q.borrow().qtype
                        });
                        let valqrtrs : Vec<_> = valid_qrtrs.collect();
                        let qnames = valqrtrs.iter()
                            .map(|ref q| q.borrow().name.clone());
                        let qnamesv : Vec<_> = qnames.collect();
                        choose_info!("a quarter for {}...",
                                     qnamesv.len() < 2, &p.name);
                        prompts::prechoose(&qnamesv, quarter_input)
                            // change the prompterror to something more informative
                            .map_err(|_| quarters::BuildError::NoQuarterFound)
                            .map(|i| valqrtrs[i].clone())
                            .map_err(Error::Build)
                    };
                    qrtr.and_then(|q| s.add_building(p, q).map_err(Error::Build))
                })
            },
            None => Err(Error::NoSett),
        }.unwrap_or_else(|e| println!("Failed to construct building: {}", e))
    }

    /// Pay n gold to repair a building.
    pub fn repair_building(&mut self, name_input: Option<String>, quarter_input: Option<String>) {
        match self.sett {
            Some(ref mut s) => {
                //TODO: rewrite repair_building to use find_buildings
                /*
                match name_input {
                    Some(b) => {
                        // get all buildings based on the input name
                        let qbldgs = s.find_buildings(b);
                    },
                    None => {
                        // get all buildings
                    },
                }
                */
                // get quarter
                // TODO: allow to skip quarter match if only one possible quarter exists for
                // building?
                let qrtr = match quarter_input.and_then(|q| s.find_quarter(&q)) {
                    Some(q) => Ok(q),
                    None => {
                        // prompt for a quarter
                        let qrtrnames = s.qrtrs.iter()
                            .map(|ref q| q.borrow().name.clone());
                        let qnamesv = qrtrnames.collect::<Vec<_>>();
                        prompts::choose(&qnamesv)
                            .map(|i| s.qrtrs[i].clone())
                            .map_err(|_| quarters::BuildError::NoQuarterFound)
                            .map_err(Error::Build)
                    },
                };
                // get building
                let bldg = qrtr.and_then(|q| {
                    match name_input {
                        Some(ref name) => {
                            // check if building exists
                            // get quarter
                            // get building
                            s.find_building(name, &q.borrow().name.clone())
                                .ok_or(quarters::BuildError::NoBuildingFound)
                        },
                        None => {
                            // prompt for a building
                            let ref bldgs = q.borrow().bldgs;
                            let bnames = bldgs.iter()
                                .map(|ref b| b.borrow().name.clone());
                            let bnamesv = bnames.collect::<Vec<_>>();
                            prompts::choose(&bnamesv)
                                .map(|i| bldgs[i].clone())
                                .map_err(|_| quarters::BuildError::NoQuarterFound)
                        },
                    }.map_err(Error::Build)
                });
                // check that we have enough gold to purchase repairs
                bldg.and_then(|b| {
                    let cost = b.borrow().get_rep_cost();
                    if let Ok(c) = cost {
                        if s.gold >= c {
                            s.gold -= c;
                            //FIXME: queues up repair for next step but caps at 100, allowing
                            //wasted value
                            b.borrow_mut().boosts.build_bonus += effects::EffectStep::new(100f64, 1);
                            Ok(())
                        } else {
                            Err(Error::Build(quarters::BuildError::NotEnoughGold))
                        }
                    } else {
                        //TODO: unhelpful as building does exist but is ruined
                        Err(Error::Build(quarters::BuildError::NoBuildingFound))
                    }
                })
            },
            None => Err(Error::NoSett),
        }.unwrap_or_else(|e| println!("Failed to repair building: {}", e))
    }

    /// Execute n settlement steps and perform all events sequentially.
    /// Write any relevant occurrences to the history.
    pub fn step(&mut self, n: i64) {
        let queue_up = match self.sett {
            Some(ref mut s) => {
                for _ in 0..n {
                    self.hist.add_entry(s.age, format!("{}", s));
                    let emap = s.step();
                    for event in emap.rand_events().iter() {
                        let ev = self.datafiles.events.iter()
                            .find(|e| e.name == *event).map(|e| e.clone());
                        if let Some(e) = ev {
                            println!("{} (step {})", e.desc.replace("{}", &s.name), s.age);
                            self.hist.add_entry(s.age, e.desc.replace("{}", &s.name));
                            self.queue.push(e);
                        }
                    }
                }
                Ok(())
            },
            None => Err(Error::NoSett),
        };
        queue_up.and_then(|_| {
            // Pop events until queue is empty.
            let activated = {
                let mut a = Ok(vec!());
                while self.queue.len() > 0 {
                    a = self.activate_event()
                }
                a
            };
            activated.map(|_| ())
        }).unwrap_or_else(|e| println!("Failed to perform step: {}", e))
    }

    /// Pop an event and perform its effects on the sett.
    /// If successful, return a Result<Vec<()>> with len == events executed.
    /// If a failure occurs, return Error::Event.
    pub fn activate_event(&mut self) -> Result<Vec<()>> {
        if let Some(e) = self.queue.pop() {
            use effects::RolledEffect as Rolled;
            let rolled = e.activate();
            rolled.iter().map(|r| {
                match *r {
                    Rolled::Kill(ref step, ref area) => {
                        match *area {
                            effects::Area::Building(ref bts) => {
                                self.rand_building(&bts).map(|b| {
                                    b.borrow_mut().boosts.grow_bonus *= step.clone(); })
                            },
                            effects::Area::Quarter(ref qts) => {
                                self.rand_quarter(&qts).map(|q| {
                                    q.borrow_mut().boosts.grow_bonus *= step.clone(); })
                            },
                            effects::Area::Sett => {
                                self.sett.as_ref().map(|s| {
                                    for q in s.qrtrs.iter() {
                                        q.borrow_mut().boosts.grow_bonus *= step.clone();
                                    }
                                })
                            },
                        }
                    },
                    Rolled::Damage(ref step, ref area) => {
                        match *area {
                            effects::Area::Building(ref bts) => {
                                self.rand_building(&bts).map(|b| {
                                    b.borrow_mut().boosts.build_bonus *= step.clone(); })
                            },
                            effects::Area::Quarter(ref qts) => {
                                self.rand_quarter(&qts).map(|q| {
                                    q.borrow_mut().boosts.build_bonus *= step.clone(); })
                            },
                            effects::Area::Sett => {
                                self.sett.as_ref().map(|s| {
                                    for q in s.qrtrs.iter() {
                                        q.borrow_mut().boosts.build_bonus *= step.clone();
                                    }
                                })
                            },
                        }
                    },
                    Rolled::Riot(ref step, ref area) => {
                        match *area {
                            effects::Area::Building(ref bts) => {
                                self.rand_building(&bts).map(|b| {
                                    b.borrow_mut().boosts.grow *= step.clone();
                                    b.borrow_mut().boosts.build *= step.clone();
                                    b.borrow_mut().boosts.gold *= step.clone(); })
                            },
                            effects::Area::Quarter(ref qts) => {
                                self.rand_quarter(&qts).map(|q| {
                                    q.borrow_mut().boosts.grow *= step.clone();
                                    q.borrow_mut().boosts.build *= step.clone();
                                    q.borrow_mut().boosts.gold *= step.clone(); })
                            },
                            effects::Area::Sett => {
                                self.sett.as_ref().map(|s| {
                                    for q in s.qrtrs.iter() {
                                        q.borrow_mut().boosts.grow *= step.clone();
                                        q.borrow_mut().boosts.build *= step.clone();
                                        q.borrow_mut().boosts.gold *= step.clone();
                                    }
                                })
                            },
                        }
                    },
                    Rolled::Grow(ref step, ref area) => {
                        match *area {
                            effects::Area::Building(ref bts) => {
                                self.rand_building(&bts).map(|b| {
                                    b.borrow_mut().boosts.grow *= step.clone(); })
                            },
                            effects::Area::Quarter(ref qts) => {
                                self.rand_quarter(&qts).map(|q| {
                                    q.borrow_mut().boosts.grow *= step.clone(); })
                            },
                            effects::Area::Sett => {
                                self.sett.as_ref().map(|s| {
                                    for q in s.qrtrs.iter() {
                                        q.borrow_mut().boosts.grow *= step.clone();
                                    }
                                })
                            },
                        }
                    },
                    Rolled::Build(ref step, ref area) => {
                        match *area {
                            effects::Area::Building(ref bts) => {
                                self.rand_building(&bts).map(|b| {
                                    b.borrow_mut().boosts.build *= step.clone(); })
                            },
                            effects::Area::Quarter(ref qts) => {
                                self.rand_quarter(&qts).map(|q| {
                                    q.borrow_mut().boosts.build *= step.clone(); })
                            },
                            effects::Area::Sett => {
                                self.sett.as_ref().map(|s| {
                                    for q in s.qrtrs.iter() {
                                        q.borrow_mut().boosts.build *= step.clone();
                                    }
                                })
                            },
                        }
                    },
                    Rolled::Gold(ref step, ref bonus) => {
                        self.sett.as_mut().map(|s| {
                            s.boosts.gold_bonus += bonus.clone();
                            s.boosts.gold *= step.clone(); })
                    },
                    Rolled::Hero(level, ref class, ref area) => {
                        let hero = self.create_hero(level, &class);
                        hero.and_then(|h| {
                            match *area {
                                effects::Area::Building(ref bts) => {
                                    self.rand_building(&bts).and_then(|b| {
                                        // TODO: loses possible error
                                        b.borrow_mut().add_occupant(h).ok() })
                                },
                                _ => None,
                            }
                        })
                    },
                    Rolled::Item(value, kind, power, ref area) => {
                        // create item
                        let name = self.namefiles.get_item();
                        let item = Rc::new(RefCell::new(
                                items::Item::new(&name, kind, power, value)));
                        // put in area
                        match *area {
                            effects::Area::Building(ref bts) => {
                                self.rand_building(&bts).and_then(|b| {
                                    // TODO: loses possible error
                                    b.borrow_mut().add_item(item).ok() })
                            },
                            _ => None,
                        }
                    },
                }.ok_or(Error::Event)
            }).collect::<Result<Vec<_>>>()
        } else {
            Ok(vec!())
        }
    }

    /// Initialize a new hero with a random name and race.
    /// Set the hero's level based on the given level integer.
    /// Select the hero's class based on the given classname string.
    fn create_hero(&self, lvl: i32, classname: &str) -> Option<Rc<RefCell<people::Hero>>> {
        let name = self.namefiles.get_hero();
        let class = self.datafiles.classes.iter()
            .find(|c| c.name == classname).map(|c| c.clone());
        class.map(|c| {
            let race = {
                let racename = rand::thread_rng().choose(&c.races)
                    .expect("Unable to create hero: the created class has no races listed!");
                racename.parse::<people::Race>()
                    .expect("Unable to create hero: one of the class's races is invalid!")
            };
            Rc::new(RefCell::new(people::Hero::new(&name, lvl, race, c)))
        })
    }

    /// Return a random quarter in the settlement.
    fn rand_quarter(&self, qtypes: &[quarters::QType]) -> Option<Rc<RefCell<quarters::Quarter>>> {
        match self.sett {
            Some(ref s) => {
                let filtered = s.qrtrs.clone().into_iter()
                    .filter(|q| qtypes.contains(&q.borrow().qtype))
                    .map(|q| q.clone()).collect::<Vec<_>>();
                rand::thread_rng().choose(&filtered).cloned()
            },
            None => None,
        }
    }

    /// Return a random building in the settlement.
    fn rand_building(&self, btypes: &[quarters::QType]) -> Option<Rc<RefCell<buildings::Building>>> {
        match self.sett {
            Some(ref s) => {
                let filtered = s.qrtrs.iter()
                    .flat_map(|q| q.borrow().bldgs.clone().into_iter())
                    .filter(|b| btypes.contains(&b.borrow().plan.btype)
                            // make sure building is in use
                            && enum_match!(b.borrow().cond, buildings::BldgCond::InUse(_)))
                    .map(|b| b.clone()).collect::<Vec<_>>();
                rand::thread_rng().choose(&filtered).cloned()
            },
            None => None,
        }
    }

    /// Check that user is willing to overwrite existing data.
    /// Return true if so, or false otherwise (and on error).
    fn confirm_overwrite() -> bool {
        prompts::bool_choose("Overwrite existing data? (y/n): ",
            &["y", "yes"], &["n", "no"]).unwrap_or(false)
    }

    /// Print information from the named term.
    /// If term is None, print the sett's information.
    pub fn print(&self, term1: Option<String>, term2: Option<String>, term3: Option<String>) {
        match term1 {
            Some(t1) => match t1.as_str() {
                "sett" => print_opt!(self.dev, self.sett),
                "quarter" => {
                    /* Displaying quarters:
                     * p quarter -> prompt, show all quarters in sett
                     * p quarter foo -> if foo is found, print it; else report not found
                     */
                    self.sett.as_ref().and_then(|s| {
                        prompts::prechoose_by_name(&s.qrtrs, term2).ok()
                    }).map(|q| dev_print!(self.dev, *q.borrow()))
                    .unwrap_or_else(|| println!("Target to print not found."));
                },
                "building" => {
                    /* Displaying buildings:
                     * p building -> prompt, show all buildings in all quarters in sett
                     * p building foo -> get all buildings named foo, if none report not found;
                     *                   if 1 print it; if >1, prompt, show all buildings found
                     * p building foo bar -> get building foo in quarter bar; if foo is found,
                     *                       print it; else report not found
                     */
                    self.sett.as_ref().and_then(|s| match term2 {
                        Some(ref t2) => {
                            let qbldgs = s.find_buildings(t2);
                            // match by possible quarters
                            let names : Vec<_> = qbldgs.iter()
                                .map(|&(ref q, _)| q).collect();
                            prompts::prechoose(&names, term3.as_ref())
                                .map(|i| qbldgs[i].1.clone()).ok()
                        },
                        None => {
                            use prompts::Described;
                            let qbldgs = s.get_buildings();
                            let names : Vec<_> = qbldgs.iter()
                                .map(|&(ref q, ref b)| {
                                    format!("{} (in {})", b.borrow().name(), q)
                                }).collect();
                            prompts::choose(&names)
                                .map(|i| qbldgs[i].1.clone()).ok()
                        },
                    }).map(|b| dev_print!(self.dev, *b.borrow()))
                    .unwrap_or_else(|| println!("Target to print not found."));
                },
                //TODO: allow prompting...
                "hero" => {
                    self.sett.as_ref().and_then(|s| match term2 {
                        Some(ref t2) => {
                            let qbheroes = s.find_heroes(t2);
                            None::<Rc<RefCell<people::Hero>>>
                        },
                        None => { None },
                    }).map(|h| dev_print!(self.dev, *h.borrow()))
                    .unwrap_or_else(|| println!("Target to print not found."));
                },
                "item" => (),
                "plans" => {
                    let plannames = self.datafiles.plans
                        .iter().map(|p| format!("{}\n", p.to_string()))
                        .collect::<String>();
                    println!("{}", plannames)
                },
                //TODO: allow second term to control history date(s)
                "history" => println!("{}", self.hist.show(None)),
                "queue" => println!("{:?}", self.queue),
                _ => (),
            },
            None => print_opt!(self.dev, self.sett),
        }
    }
}
