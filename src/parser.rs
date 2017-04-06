//! Parse the JSON data files used to store settlement information.
//! The get_data function deserializes the data to the intended struct.
//!
//! ## Example
//!
//! ```
//! use podesta::parser;
//! use podesta::events;
//! let events : Vec<events::Event> = parser::get_data("events.json").
//!     expect("Failed to get data");
//! ```

use serde;
use serde_json;

use std::fs::File;
use std::io::BufReader;
use std::env;
use std::path;
use std::rc::Rc;

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
    let e: Vec<Rc<T>> = try!(serde_json::from_reader(reader));
    Ok(e)
}

/// Return the PathBuf to the directory where the data is stored.
/// This is $CARGO_MANIFEST_DIR/lib/data/.
fn get_data_dir() -> path::PathBuf {
    let head = env::var_os("CARGO_MANIFEST_DIR")
        .expect("Must run using Cargo!");
    let p = path::Path::new(&head).join("lib").join("data");

    p
}
