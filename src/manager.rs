//! The manager for a settlement
//! Tracks metavariables and manages the event queue and its effects on the
//! settlement's members.

use parser;
use time;
use sett;
use quarters;
use buildings;
use people;
use history;
use events;
use prompts;
use std::fmt;
use std::error;
use std::cell::RefCell;
use std::rc::Rc;

macro_rules! choose_info {
    ($printexpr:expr, $auto:expr, $name:expr) => {
        print!("Choos{} ", if $auto { "ing" } else { "e" });
        println!($printexpr, $name);
    };
}

macro_rules! print_opt {
    ($opt:expr) => {
        match $opt {
            Some(ref x) => println!("{}", x),
            None => (),
        }
    };
}

#[derive(Debug)]
pub enum Error {
    Data(parser::GameDataError),
    Build(quarters::BuildError),
    NoSett,
    History,
    Event,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Data(ref e) => e.fmt(f),
            Error::Build(ref e) => e.fmt(f),
            Error::NoSett => write!(f, "No sett found (first run 'new' or 'load')"),
            Error::History => write!(f, "Failed to update history log"),
            Error::Event => write!(f, "Failed to perform event"),
        }
    }
}

impl From<parser::GameDataError> for Error {
    fn from(err: parser::GameDataError) -> Error { Error::Data(err) }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Data(ref err) => err.description(),
            Error::Build(ref err) => err.description(),
            Error::NoSett => "no sett found",
            Error::History => "unable to write history",
            Error::Event => "unable to perform event",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Data(ref err) => Some(err),
            Error::Build(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<quarters::BuildError> for Error {
    fn from(err: quarters::BuildError) -> Error { Error::Build(err) }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manager {
    datafiles: parser::DataFiles,
    sett: Option<sett::Sett>,
    hist: history::History,
    queue: events::EventQueue,
    automate: bool,
    verbose: bool,
    savefile: String,
}

impl Manager {
    /// Create a new Manager with the given data files.
    /// Note that until build_sett() is called, no settlement actually exists.
    pub fn new(reg_path: &str, bldg_path: &str, ev_path: &str,
               verb: bool) -> Self {
        Manager {
            datafiles: parser::DataFiles::new(reg_path, bldg_path, ev_path),
            sett: None,
            hist: history::History::new(),
            queue: events::EventQueue::new(32),
            automate: false,
            verbose: verb,
            savefile: format!("pod-{}.rbs", time::now().ctime()),
        }
    }

    /// Load a Manager from a file with the given name; if None is given,
    /// prompt the user for one.
    pub fn load(file: Option<String>) -> Result<Self, parser::GameDataError> {
        match file {
            Some(filename) => Ok(filename),
            None => prompts::name_file(" to load: "),
        }.map_err(parser::GameDataError::Prompt)
            .and_then(|f| parser::load_rbs(f.as_str()))

    }

    /// Save a Manager to its save file, optionally prompting the user for a
    /// file name or accepting one.
    pub fn save(&self, file: Option<String>) -> Result<(), parser::GameDataError> {
        // Use the given file, OR prompt for a file, OR use the default
        let savef = file.unwrap_or(
            prompts::name_file(&format!(" to save to (default: {}): ",
                                        self.savefile))
                               .unwrap_or(self.savefile.clone()));
        parser::save_rbs(self, &savef)
    }
    /// Toggle automation of build functions.
    pub fn toggle_auto(&mut self) {
        self.automate = !self.automate;
        if self.verbose { println!("Automation set to {}", self.automate) }
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
                s.add_quarter(name.clone(), qtype, race)
                    .unwrap_or_else(|e| {
                    println!("Failed to construct {} quarter: {}",
                             name, e);
                    })
            },
            None => println!("No sett found (first run 'new' or 'load')!"),
        }
    }

    /// Initialize a new building and store it in the manager's sett's quarter.
    #[allow(unused_variables)]
    pub fn build_building(&mut self,
                          name_input: Option<String>,
                          quarter_input: Option<String>) -> Result<(), Error>
    {
        let plan = match self.sett {
            Some(ref mut s) => {
                // Determine building plan
                let ref plans = self.datafiles.plans;
                // &String iterator of plan names
                let mut plannames = plans.iter().map(|ref p| &p.name);
                // Find index of plan to add
                match name_input {
                    Some(s) => plannames.position(|x| x == &s)
                        .ok_or(quarters::BuildError::NoPlanFound),
                    None => { prompts::choose(&plannames.collect::<Vec<_>>())
                        .map_err(parser::GameDataError::Prompt).map_err(|e| {
                            println!("Error choosing building: {:?}", e);
                            quarters::BuildError::NoPlanFound
                        })
                    },
                }.map(|i| plans[i].clone()).map_err(Error::Build)

                /*
                match plan {
                    Ok(b) => {
                        let q = self.get_quarter(&b, quarter_input);
                    }
                }
                */
                // Get quarter where building should be added.
                // Call s.add_building() for that quarter.
                /*
                plan.and_then(|ref b| {
                    self.get_quarter(&b, quarter_input)
                        .and_then(|q| {
                            s.add_building(b.clone(), q).map_err(Error::Build)
                        })
                })
                */
            },
            None => Err(Error::NoSett),
        };//.and_then(|ref b| { println!("Found a plan!") })
            // get the quarter
            // add it to the settlement
        plan.map(|_| println!("Found a plan!"))
    }

    /*
    /// Get a reference-counted pointer to a quarter matching the provided
    /// information (building plan and optional input string).
    fn get_quarter(&self, bplan: &buildings::BuildingPlan, input: Option<String>)
        -> Result<Rc<RefCell<quarters::Quarter>>, Error>
    {
        let s = self.sett.unwrap();
        // Select quarters where the building could be constructed
        let mut quarters = s.qrtrs.into_iter();
        let valid_qrtrs = quarters.by_ref().filter(|ref q| {
            bplan.btype == q.borrow().qtype
        });
        // Confirm which quarter should host the building
        let mut qnames = valid_qrtrs.map(|ref q| q.borrow().name.clone());
        match input {
            Some(q) => qnames.position(|x| x == q)
                .ok_or(quarters::BuildError::NoQuarterFound),
            None => {
                let qnamesv = qnames.collect::<Vec<_>>();
                // Cleanly ask which quarter should host the building
                match qnamesv.len() {
                    0 => Err(quarters::BuildError::NoQuarterFound),
                    1 => Ok(0),
                    _ => {
                        println!("Please specify where \
                                 to build the building:");
                        prompts::choose(&qnamesv)
                        .map_err(|e| {
                            println!("Error choosing quarter: {:?}", e);
                            quarters::BuildError::NoQuarterFound
                        })
                    },
                }
            },
        }.map(|i| s.qrtrs[i].clone()).map_err(Error::Build)
    }
    */

    /// Execute n settlement steps and perform all events sequentially.
    /// Write any relevant occurrences to the history.
    pub fn step(&mut self, n: i64) {
        match self.sett {
            Some(ref mut s) => {
                for _ in 0..n {
                    self.hist.add_entry(format!("Step {}", s.age),
                                        format!("{}", s));
                    s.step();
                }
            },
            None => println!("No sett found (first run 'new' or 'load')!"),
        }
    }

    /// Pop an event and perform its effects on the sett.
    pub fn activate_event(&mut self) {
        if let Some(e) = self.queue.pop() {
            let effects = e.effects;
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
    pub fn print(&self, term: Option<String>) {
        match term {
            Some(t) => match t.as_str() {
                "sett" => print_opt!(self.sett),
                "quarter" => (),
                "building" => (),
                _ => (),
            },
            None => print_opt!(self.sett),
        }
    }
}
