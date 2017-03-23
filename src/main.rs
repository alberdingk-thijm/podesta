#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

mod parser;
mod events;
mod buildings;
//use std::io;

fn main() {

    let events : Vec<events::Event> = parser::get_data("events.json")
        .expect("Error parsing JSON!");

    println!("Loaded events.json! {} events found", events.len());

    let buildings : Vec<buildings::Building> = parser::get_data("buildings.json")
        .expect("Error parsing JSON!");

    println!("Loaded buildings.json! {} buildings found", buildings.len());

    println!("{:?}", buildings[0]);

    let ev = events::Event {
        name: "Fire".to_string(),
        id: 1,
        desc: "%s has erupted into flames!".to_string(),
        chance: 3,
        event: vec!("Kill_2".to_string(), "Dam_2".to_string()),
    };

    let serialized = serde_json::to_string(&ev).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: events::Event = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
