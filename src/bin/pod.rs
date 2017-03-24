#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

extern crate podesta;

fn main() {

    let events : Vec<podesta::events::Event> = podesta::parser::get_data("events.json")
        .expect("Error parsing JSON!");

    println!("Loaded events.json! {} events found", events.len());

    let buildings : Vec<podesta::buildings::Building> = podesta::parser::get_data("buildings.json")
        .expect("Error parsing JSON!");

    println!("Loaded buildings.json! {} buildings found", buildings.len());

    let regions : Vec<podesta::regions::Region> = podesta::parser::get_data("regions.json")
        .expect("Error parsing JSON!");

    println!("Loaded regions.json! {} regions found", regions.len());

    let ev = podesta::events::Event {
        name: "Fire".to_string(),
        id: 1,
        desc: "%s has erupted into flames!".to_string(),
        chance: 3,
        effects: vec!("Kill_2".to_string(), "Dam_2".to_string()),
    };

    let serialized = serde_json::to_string(&ev).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: podesta::events::Event = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    podesta::init();
}
