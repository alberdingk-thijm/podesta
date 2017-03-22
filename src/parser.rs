use serde;
use serde_json;

use std::fs::File;
use std::io::BufReader;
use std::env;

pub fn get_data<T>(jsonfile: &str) -> Result<Vec<T>, serde_json::Error>
    where T: serde::Deserialize + serde::Serialize {

    let mut p = env::current_dir().unwrap();
    p.push("lib");
    p.push("data");
    p.push(jsonfile);
    let f = File::open(p)
        .expect("Unable to open file");
    let reader = BufReader::new(f);
    let e: Vec<T> = try!(serde_json::from_reader(reader));
    Ok(e)
}
