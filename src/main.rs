#[macro_use]
extern crate serde_derive;
extern crate serde_json;

//use std::io;
use std::fs::File;
use std::io::{Read, BufReader};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    name: String,
    id: i32,
    desc: String,
    chance: i32,
    event: Vec<String>,
    // TODO: convert to Vec<event effect>
}

#[allow(dead_code)]
fn get_events() -> Result<Vec<Event>, serde_json::Error> {
    // TODO: fix file path
    let mut p = env::current_dir().unwrap();
    p.push("lib");
    p.push("data");
    p.push("events.json");
    let mut f = File::open(p)
        .expect("Unable to open events file!");
    let mut reader = BufReader::new(f);

    let mut e: Vec<Event> = try!(serde_json::from_reader(reader));

    Ok(e)

}

fn main() {

    match get_events() {
        Ok(e) => { let events = e; println!("Loaded events.json!") },
        Err(e) => println!("Error: {}", e.to_string()),
    }

    let ev = Event {
        name: "FIRE".to_string(),
        id: 1,
        desc: "%s has erupted into flames!".to_string(),
        chance: 3,
        event: vec!("KILL_2".to_string(), "DAM_2".to_string()),
    };

    let serialized = serde_json::to_string(&ev).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Event = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
