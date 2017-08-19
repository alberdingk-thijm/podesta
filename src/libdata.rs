//! Parse the JSON data files used to store settlement information.
//! The get_data function deserializes the data to the intended struct.
//!
//! ## Example
//!
//! ```
//! use podesta::parser;
//! use podesta::events;
//! use std::rc::Rc;
//! let events : Vec<Rc<events::Event>> = parser::get_data("events.json").
//!     expect("Failed to get data");
//! ```

use serde;
use serde_json;
use bincode;
use regions::Region;
use buildings::BuildingPlan;
use events::Event;
use people::Class;
use manager;
use prompts::PromptError;
use rand::{self, Rng};

use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::fmt;
use std::error;

/// A structure for listing file paths.
#[derive(Debug)]
pub struct PathList {
    /// Regions (data)
    pub regs: PathBuf,
    /// Building plans (data)
    pub bldgs: PathBuf,
    /// Events (data)
    pub evs: PathBuf,
    /// Classes (data)
    pub cls: PathBuf,
    /// People (names)
    pub pep: PathBuf,
    /// Items (names)
    pub its: PathBuf,
    /// Adjectives (names)
    pub adjs: PathBuf,
}

impl PathList {
    /// Create a new PathList.
    fn new<P: AsRef<Path>>(r: P, b: P, e: P, c: P, p: P, i: P, a: P) -> PathList {
        PathList {
            regs: r.as_ref().to_path_buf(),
            bldgs: b.as_ref().to_path_buf(),
            evs: e.as_ref().to_path_buf(),
            cls: c.as_ref().to_path_buf(),
            pep: p.as_ref().to_path_buf(),
            its: i.as_ref().to_path_buf(),
            adjs: a.as_ref().to_path_buf(),
        }
    }

    /// Generate a new PathList from the given data and names directories.
    pub fn from_dirs<P>(data: P, names: P) -> Result<PathList, LibError>
    where P: AsRef<Path>
    {
        // check that data and names are both directories
        let data : PathBuf = data.as_ref().to_path_buf();
        let names : PathBuf = names.as_ref().to_path_buf();
        if !(data.is_dir() && names.is_dir()) {
            // TODO: add to LibError
            return Err(LibError::InvalidPath)
        }
        // Get all json files in each directory
        let (regs, bldgs, evs, cls) = {
            (data.join("regions.json"),
             data.join("buildings.json"),
             data.join("events.json"),
             data.join("classes.json"))};
        let (pep, its, adjs) = {
            (names.join("people.txt"),
             names.join("items.txt"),
             names.join("adjectives.txt"))};
        Ok(PathList::new(regs, bldgs, evs, cls, pep, its, adjs))
    }
}

/// A structure for storing game data extracted from files (lib/data/)
#[derive(Debug, Serialize, Deserialize)]
pub struct DataFiles {
    pub regions: Vec<Rc<Region>>,
    pub plans: Vec<Rc<BuildingPlan>>,
    pub events: Vec<Rc<Event>>,
    pub classes: Vec<Rc<Class>>,
}

/// A structure for storing name data extracted from files (lib/names/)
#[derive(Debug, Serialize, Deserialize)]
pub struct NameFiles {
    pub people: Vec<String>,
    pub items: Vec<String>,
    pub adjectives: Vec<String>,
}

impl DataFiles {
    /// Create a new DataFiles struct to track regions, buildings, events,
    /// and classes.
    /// NOTE: Be mindful of the order when providing the parameters!
    pub fn new(region_path: &Path,
               building_path: &Path,
               event_path: &Path,
               class_path: &Path) -> DataFiles {
        DataFiles {
            regions: get_data(region_path).unwrap(),
            plans: get_data(building_path).unwrap(),
            events: get_data(event_path).unwrap(),
            classes: get_data(class_path).unwrap()
        }
    }

    pub fn from_pathlist(pl: &PathList) -> DataFiles {
        DataFiles::new(&pl.regs, &pl.bldgs, &pl.evs, &pl.cls)
    }
}

impl NameFiles {
    pub fn new(people_path: &Path, items_path: &Path, adj_path: &Path)
        -> NameFiles {
        NameFiles {
            people: get_names(people_path).unwrap(),
            items: get_names(items_path).unwrap(),
            adjectives: get_names(adj_path).unwrap()
        }
    }

    pub fn from_pathlist(pl: &PathList) -> NameFiles {
        NameFiles::new(&pl.pep, &pl.its, &pl.adjs)
    }

    /// Get a random item name using a particular style
    pub fn get_item(&self) -> String {
        let desc = rand::thread_rng().choose(&self.adjectives)
            .expect("adjectives namefile has no elements!");
        let name = rand::thread_rng().choose(&self.people)
            .expect("people namefile has no elements!");
        format!("{} {}", desc, name)
    }

    /// Get a random hero name using a particular style
    pub fn get_hero(&self) -> String {
        let name = rand::thread_rng().choose(&self.people)
            .expect("people namefile has no elements!");
        let epithet = rand::thread_rng().choose(&self.adjectives)
            .expect("adjectives namefile has no elements!");
        format!("{} the {}", name, epithet)
    }
}

/// Return a Result holding a deserialized vector of podsim data,
/// where each vector element was stored in a JSON file named by
/// the **jsonfile** parameter; or an error if the JSON file could
/// not be parsed. Each element must itself be deserializable.
/// Panic! if the file cannot be opened.
pub fn get_data<T>(jsonfile: &Path) -> Result<Vec<Rc<T>>, LibError>
where
    T: serde::Deserialize + serde::Serialize
{
    /*
    let mut p = get_dir("data");
    p.push(jsonfile);
    let f = File::open(p)
        .expect("Unable to open file");
    */
    let f = File::open(jsonfile)?;
    let reader = BufReader::new(f);
    let e: Vec<Rc<T>> = serde_json::from_reader(reader)?;
    Ok(e)
}

/// Return a vector of strs from the given text file.
pub fn get_names(txtfile: &Path) -> Result<Vec<String>, LibError> {
    let f = File::open(txtfile)?;
    let reader = BufReader::new(f);
    reader.lines().collect::<io::Result<Vec<String>>>()
        .map_err(LibError::Io)
}

#[derive(Debug)]
pub enum LibError {
    Serde(serde_json::Error),
    Bincode(bincode::Error),
    Io(io::Error),
    Prompt(PromptError),
    InvalidPath,
}

impl From<io::Error> for LibError {
    fn from(err: io::Error) -> LibError { LibError::Io(err) }
}

impl From<serde_json::Error> for LibError {
    fn from(err: serde_json::Error) -> LibError { LibError::Serde(err) }
}

impl From<bincode::Error> for LibError {
    fn from(err: bincode::Error) -> LibError { LibError::Bincode(err) }
}

impl From<PromptError> for LibError {
    fn from(err: PromptError) -> LibError { LibError::Prompt(err) }
}

impl fmt::Display for LibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LibError::Serde(ref err) => err.fmt(f),
            LibError::Bincode(ref err) => err.fmt(f),
            LibError::Io(ref err) => err.fmt(f),
            LibError::Prompt(ref err) => err.fmt(f),
            LibError::InvalidPath => write!(f, "Invalid path given"),
        }
    }
}

impl error::Error for LibError {
    fn description(&self) -> &str {
        match *self {
            LibError::Serde(ref err) => err.description(),
            LibError::Bincode(ref err) => err.description(),
            LibError::Io(ref err) => err.description(),
            LibError::Prompt(ref err) => err.description(),
            LibError::InvalidPath => "invalid path",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LibError::Serde(ref err) => err.cause(),
            LibError::Bincode(ref err) => err.cause(),
            LibError::Io(ref err) => err.cause(),
            LibError::Prompt(ref err) => err.cause(),
            LibError::InvalidPath => None,
        }
    }
}

/// Save the manager to a given .rbs file name.
///
/// # Example
///
/// ```
/// use podesta::parser;
/// # use podesta::manager::Manager;
/// # let man = Manager::new("regions.json",
///     "buildings.json", "events.json", false);
/// parser::save_rbs(&man, "foo.rbs").unwrap()
/// ```
pub fn save_rbs(man: &manager::Manager, fname: &str) -> Result<(), LibError> {
    let fullname = format!("{}{}", fname,
                           if !fname.ends_with(".rbs") { ".rbs" } else { "" });
    let f = File::create(fullname)?;
    let mut writer = BufWriter::new(f);
    // serialize the manager using bincode
    bincode::serialize_into(&mut writer, man, bincode::Infinite)
        .map_err(LibError::Bincode)
}

/// Load a manager from a given .rbs file.
///
/// # Example
///
/// ```
/// use podesta::parser;
/// # use podesta::manager::Manager;
/// # let man = Manager::new("regions.json",
///     "buildings.json", "events.json", false);
/// # parser::save_rbs(&man, "foo.rbs").unwrap()
/// let man = parser::load_rbs("foo.rbs").unwrap();
/// ```
pub fn load_rbs(fname: &str) -> Result<manager::Manager, LibError> {
    let fullname = format!("{}{}", fname,
                           if !fname.ends_with(".rbs") { ".rbs" } else { "" });
    let f = File::open(fullname)?;
    let mut reader = BufReader::new(f);
    // deserialize the manager using bincode
    bincode::deserialize_from(&mut reader, bincode::Infinite)
        .map_err(LibError::Bincode)
}
