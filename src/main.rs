#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

//use std::io;
use std::fs::File;
use std::io::BufReader;
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

#[derive(Serialize, Deserialize, Debug)]
struct Building {
    name: String,
    id: i32,
    btype: BType,  // use serde macro to match type
    preq: Option<Vec<String>>,  // can be null
    cost: f32,
    build: f32,
    events: Vec<EventChance>,
    flags: i32,
}

#[derive(Serialize, Deserialize, Debug)]
enum BType {
    Residential,
    Industrial,
    Port,
    Academic,
    Administrative,
}

#[derive(Serialize, Deserialize, Debug)]
struct EventChance {
    name: String,
    chance: f32,
}


fn get_data<T>(jsonfile: &str) -> Result<Vec<T>, serde_json::Error>
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

fn main() {


    let events : Vec<Event> = get_data("events.json").expect("Error parsing JSON!");

    println!("Loaded events.json!");

    for ev in events.into_iter() {
        println!("{}", ev.name);
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
