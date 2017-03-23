#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

pub mod parser;
pub mod events;
pub mod buildings;
pub mod regions;
pub mod quarters;
pub mod people;

#[allow(unused_variables)]
#[test]
fn get_datafiles() {

    let events : Vec<events::Event> =
        parser::get_data("events.json").expect("Error parsing JSON!");

    let buildings : Vec<buildings::Building> =
        parser::get_data("buildings.json").expect("Error parsing JSON!");

    let regions : Vec<regions::Region> =
        parser::get_data("regions.json").expect("Error parsing JSON!");
}

/// Create a new settlement, prompting for user input occasionally.
fn init() {
    // Prompt for region type
    // Generate empty settlement
    //
    unimplemented!()
}
