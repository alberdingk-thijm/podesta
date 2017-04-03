extern crate libc;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate rouler;
extern crate rand;

pub mod parser;
pub mod regions;
pub mod sett;
pub mod buildings;
pub mod quarters;
pub mod people;
//pub mod events;
//pub mod effects;

#[allow(unused_variables)]
#[test]
fn get_datafiles() {

    /*
    let events : Vec<events::Event> =
        parser::get_data("events.json").expect("Error parsing JSON!");
        */

    let plans : Vec<buildings::BuildingPlan> =
        parser::get_data("buildings.json").expect("Error parsing JSON!");

    let regions : Vec<regions::Region> =
        parser::get_data("regions.json").expect("Error parsing JSON!");
}

/// Create a new settlement, prompting for user input occasionally.
pub fn init() {
    // Prompt for region
    let reg = regions::Region {
        name: "Floodplain".to_string(),
        desc: "marshy wetlands".to_string(),
        growth: 6,
        starting_gold: 100.0,
    };
    // Prompt for quarter type
    let qtype = quarters::QType::Port;
    // Prompt for race?
    let race = people::Race::Human;
    // Generate empty settlement
    let s = sett::Sett::new("Amsterdam", reg, qtype, race, sett::SettFlags::Coastal);
    println!("Generated a settlement: {:?}", s);
}

pub const WELCOME_MINI : &'static str = r#"
Welcome to Podesta v0.1!
Type "help" or "license" for more info.
Type "q" to exit.
"#;

/// Display the help file
pub fn help() {
    unimplemented!()
}

/// Display the license file
pub fn license() {
    unimplemented!()
}
