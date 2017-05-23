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
use manager;
use prompts::PromptError;

use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::env;
use std::path;
use std::rc::Rc;
use std::fmt;
use std::error;

/// A structure for storing game data extracted from files
#[derive(Debug, Serialize, Deserialize)]
pub struct DataFiles {
    pub regions: Vec<Rc<Region>>,
    pub plans: Vec<Rc<BuildingPlan>>,
    pub events: Vec<Rc<Event>>,
}

macro_rules! load_datafile {
    ($ftype:expr, $path:expr) => {
        get_data($path).and_then(|d| {
            println!("Loaded {}! {} {} found.", $path, d.len(), $ftype);
            Ok(d)
        }).expect("Error parsing JSON!")
    };
}

impl DataFiles {
    /// Create a new DataFiles struct to track regions, buildings, and
    /// (eventually) events.
    /// NOTE: Be mindful of the order when providing the parameters!
    pub fn new(region_path: &str,
               building_path: &str,
               event_path: &str) -> DataFiles {
        DataFiles {
            regions: load_datafile!("regions", region_path),
            plans: load_datafile!("buildings", building_path),
            events: load_datafile!("events", event_path)
        }
    }
}

/// Return a Result holding a deserialized vector of podsim data,
/// where each vector element was stored in a JSON file named by
/// the **jsonfile** parameter; or an error if the JSON file could
/// not be parsed. Each element must itself be deserializable.
/// Panic! if the file cannot be opened.
pub fn get_data<T>(jsonfile: &str) -> Result<Vec<Rc<T>>, serde_json::Error>
where
    T: serde::Deserialize + serde::Serialize
{
    let mut p = get_data_dir();
    p.push(jsonfile);
    let f = File::open(p)
        .expect("Unable to open file");
    let reader = BufReader::new(f);
    let e: Vec<Rc<T>> = serde_json::from_reader(reader)?;
    Ok(e)
}

/// Return the PathBuf to the directory where the data is stored.
/// This is $CARGO_MANIFEST_DIR/lib/data/.
/// Will panic if $CARGO_MANIFEST_DIR is not set.
fn get_data_dir() -> path::PathBuf {
    let head = env::var_os("CARGO_MANIFEST_DIR")
        .expect("Must run using Cargo!");
    let p = path::Path::new(&head).join("lib").join("data");
    p
}

#[derive(Debug)]
pub enum GameDataError {
    Serde(serde_json::Error),
    Bincode(bincode::Error),
    Io(io::Error),
    Prompt(PromptError),
}

impl From<io::Error> for GameDataError {
    fn from(err: io::Error) -> GameDataError { GameDataError::Io(err) }
}

impl From<serde_json::Error> for GameDataError {
    fn from(err: serde_json::Error) -> GameDataError { GameDataError::Serde(err) }
}

impl From<bincode::Error> for GameDataError {
    fn from(err: bincode::Error) -> GameDataError { GameDataError::Bincode(err) }
}

impl From<PromptError> for GameDataError {
    fn from(err: PromptError) -> GameDataError { GameDataError::Prompt(err) }
}

impl fmt::Display for GameDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameDataError::Serde(ref err) => err.fmt(f),
            GameDataError::Bincode(ref err) => err.fmt(f),
            GameDataError::Io(ref err) => err.fmt(f),
            GameDataError::Prompt(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for GameDataError {
    fn description(&self) -> &str {
        match *self {
            GameDataError::Serde(ref err) => err.description(),
            GameDataError::Bincode(ref err) => err.description(),
            GameDataError::Io(ref err) => err.description(),
            GameDataError::Prompt(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            GameDataError::Serde(ref err) => err.cause(),
            GameDataError::Bincode(ref err) => err.cause(),
            GameDataError::Io(ref err) => err.cause(),
            GameDataError::Prompt(ref err) => err.cause(),
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
pub fn save_rbs(man: &manager::Manager, fname: &str) -> Result<(), GameDataError> {
    let fullname = format!("{}{}", fname,
                           if !fname.ends_with(".rbs") { ".rbs" } else { "" });
    let f = File::create(fullname)?;
    let mut writer = BufWriter::new(f);
    // serialize the manager using bincode
    bincode::serialize_into(&mut writer, man, bincode::Infinite)
        .map_err(GameDataError::Bincode)
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
pub fn load_rbs(fname: &str) -> Result<manager::Manager, GameDataError> {
    let fullname = format!("{}{}", fname,
                           if !fname.ends_with(".rbs") { ".rbs" } else { "" });
    let f = File::open(fullname)?;
    let mut reader = BufReader::new(f);
    // deserialize the manager using bincode
    bincode::deserialize_from(&mut reader, bincode::Infinite)
        .map_err(GameDataError::Bincode)
}

