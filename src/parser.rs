use serde;
use serde_json;

use std::fs::File;
use std::io::BufReader;
use std::env;
use std::path;

//const DATAPATH: &'static str = concat!("lib", path::MAIN_SEPARATOR, "data");

pub fn get_data<T>(jsonfile: &str) -> Result<Vec<T>, serde_json::Error>
    where T: serde::Deserialize + serde::Serialize {

    /*
    let mut p = env::current_dir().unwrap();
    p.push("lib");
    p.push("data");
    */
    let mut p = get_data_dir();
    p.push(jsonfile);
    let f = File::open(p)
        .expect("Unable to open file");
    let reader = BufReader::new(f);
    let e: Vec<T> = try!(serde_json::from_reader(reader));
    Ok(e)
}

// return the PathBuf to the directory where the data is stored
// ($CARGO_MANIFEST_DIR/lib/data/)
fn get_data_dir() -> path::PathBuf {
    let head = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let p = path::Path::new(&head).join("lib").join("data");

    p
}
