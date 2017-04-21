/// The manager for a settlement
/// Tracks metavariables and manages the event queue and its effects on the
/// settlement's members.

use parser;
use sett;
use quarters;
//use buildings;
use people;
use history;
use events;
use prompts;

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
#[derive(Debug, Serialize, Deserialize)]
pub struct Manager {
    datafiles: parser::DataFiles,
    sett: Option<sett::Sett>,
    hist: history::History,
    queue: events::EventQueue,
    automate: bool,
    verbose: bool,
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
        }
    }

    /// Load a Manager from a file with the given name; if None is given,
    /// prompt the user for one.
    /// Return None if an error occurs.
    pub fn load(file: Option<String>) -> Result<Self, parser::GameDataError> {
        match file {
            Some(filename) => Ok(filename),
            None => prompts::name_file(),
        }.map_err(parser::GameDataError::Prompt)
            .and_then(|f| parser::load_rbs(f.as_str()))

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

            self.sett = Some(
                sett::Sett::new(name, reg, qtype, race, coastchoice)
            );

            if self.verbose { println!("{}", self.sett.as_ref().unwrap()) }
        }
    }

    /// Return the name of the sett, or None if no sett exists.
    pub fn get_sett_name(&self) -> Option<String> {
        self.sett.as_ref().map(|ref s| s.name.clone())
    }

    /// Initialize a new quarter and store it in the manager's sett.
    #[allow(unused_variables)]
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
                choose_info!("the focus of {}'s main quarter...",
                             self.automate, name);
                let qchoice = prompts::choose_or_rand(
                    &quarters::QType::get_qtype_names(s.coastal),
                    nprompts);
                let qtype = quarters::QType::get_qtypes(s.coastal)[qchoice];
                // Race
                choose_info!("the majority race of {}'s main quarter...",
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
    pub fn build_building(&mut self, name_input: Option<String>) {
        unimplemented!()
    }

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
        unimplemented!()
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
