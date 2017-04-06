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
mod prompts;
//pub mod events;
//pub mod effects;

use std::rc::Rc;

#[derive(Debug)]
pub struct DataFiles {
    regions: Vec<Rc<regions::Region>>,
    plans: Vec<Rc<buildings::BuildingPlan>>,
    //events: Vec<events::Event>,
}

impl DataFiles {
    /// Create a new DataFiles struct to track regions, buildings, and
    /// (eventually) events.
    /// NOTE: Be mindful of the order when providing the parameters!
    pub fn new(region_path: &str, building_path: &str) -> DataFiles {
        DataFiles {
            regions: parser::get_data(region_path)
                .and_then(|d| {
                    println!("Loaded {}! {} regions found.",
                                       region_path, d.len());
                    Ok(d)
                }).expect("Error parsing regions JSON!"),
            plans: parser::get_data(building_path)
                .and_then(|d| {
                    println!("Loaded {}! {} buildings found.",
                                       building_path, d.len());
                    Ok(d)
                }).expect("Error parsing buildings JSON!"),
        }
    }
}

#[test]
fn get_datafiles() {
    /*
    let _events : Vec<events::Event> =
        parser::get_data("events.json").expect("Error parsing JSON!");
        */
    let _data = DataFiles::new("regions.json", "buildings.json");
}

/// Create a new settlement, prompting for user input occasionally.
pub fn init(data: &DataFiles) {
    // Prompt for a name of at least 1 character.
    let namelen = 1;
    let mut namechoice = prompts::name(namelen);
    while namechoice.is_err() {
        println!("Please enter at least {} letters.", namelen);
        namechoice = prompts::name(namelen);
    }
    let name = namechoice.unwrap();
    // Prompt for region (max 2 tries)
    let regchoice = prompts::choose_or_rand(&(data.regions), 2);
    let reg = data.regions[regchoice].clone();
        //TODO: cannot move elements inside data.regions (wrap with Box?)
        //.into_iter().nth(regchoice).expect("Unable to get region!");
    // Prompt for quarter type
    let qtype = quarters::QType::Port;
    // Prompt for race?
    let race = people::Race::Human;
    // Generate empty settlement
    let mut s = sett::Sett::new(name,
                                reg,
                                qtype,
                                race,
                                sett::SettFlags::Coastal);
    println!("Generated a settlement: {:?}", s);

    println!("{:?}", s.add_quarter("Jewish Quarter".to_string(),
                  quarters::QType::Industrial,
                  people::Race::Human));

    println!("Settlement state: {:?}", s);
}

pub const WELCOME_MINI : &'static str = r#"
Welcome to Podesta v0.1!
Type "help" or "license" for more info.
Type "commands" to list some basic commands.
Type "q" to exit.
"#;

pub const COMMANDS : &'static str = r#"
help            -   view help file
license         -   view license file
new [term]      -   create a new [term]
step, n, next   -   execute a step
p, print [term] -   print [term]
save [file]     -   save the settlement to file
load [file]     -   load a settlement from a file
"#;

/// Display the help file
pub fn help() {
    unimplemented!()
}

/// Display the license file
pub fn license() {
    unimplemented!()
}
