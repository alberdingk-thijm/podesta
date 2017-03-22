#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

mod parser;
mod events;
use std::env;
//use std::io;

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


fn main() {

    let events : Vec<events::Event> = parser::get_data("events.json")
        .expect("Error parsing JSON!");

    println!("Loaded events.json! {} events found", events.len());

    let ev = events::Event {
        name: "FIRE".to_string(),
        id: 1,
        desc: "%s has erupted into flames!".to_string(),
        chance: 3,
        event: vec!("KILL_2".to_string(), "DAM_2".to_string()),
    };

    let serialized = serde_json::to_string(&ev).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: events::Event = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
