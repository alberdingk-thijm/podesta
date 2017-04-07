extern crate libc;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate rouler;
extern crate rand;
#[macro_use] extern crate macro_attr;
#[macro_use] extern crate enum_derive;

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
pub fn new_sett(data: &DataFiles, auto: bool) {
    let numprompts = if auto { 0 } else { 2 };
    // Name
    let namelen = 1;  // at least one character
    let mut namechoice = prompts::name(namelen);
    while namechoice.is_err() {
        println!("Please enter at least {} letters.", namelen);
        namechoice = prompts::name(namelen);
    }
    let name = namechoice.unwrap();
    // Region
    println!("Choos{} {}'s region...",
             if auto { "ing" } else { "e" }, name);
    let regchoice = prompts::choose_or_rand(&(data.regions), numprompts);
    let reg = data.regions[regchoice].clone();

    // Quarter type
    println!("Choos{} the focus of {}'s main quarter...",
             if auto { "ing" } else { "e" }, name);
    let qtypenames = quarters::QType::iter_variant_names();
    let qchoice = prompts::choose_or_rand(
        &(qtypenames.collect::<Vec<_>>()), numprompts);
    let qtype = quarters::QType::iter_variants().nth(qchoice).unwrap();

    // Race
    println!("Choos{} the majority race of {}'s main quarter...",
             if auto { "ing" } else { "e" }, name);
    let racenames = people::Race::iter_variant_names();
    let racechoice = prompts::choose_or_rand(
        &(racenames.collect::<Vec<_>>()), numprompts);
    let race = people::Race::iter_variants().nth(racechoice).unwrap();
    // Flags
    println!("Choos{} if {} is coastal...",
             if auto { "ing" } else { "e" }, name);

    // Generate empty settlement
    let s = sett::Sett::new(name,
                            reg,
                            qtype,
                            race,
                            sett::SettFlags::Coastal);
    println!("Generated a settlement: {}", s);
}

pub const WELCOME_MINI : &'static str = r#"
Welcome to Podesta v0.1!
Type "help" or "license" for more info.
Type "commands" to list some basic commands.
Type "q" to quit.
"#;

pub const COMMANDS : &'static str = r#"
help            -   view help file
license         -   view license file
a, auto         -   toggle automatic creation and stepping
new [term]      -   create a new [term]
step, n, next   -   execute a step
p, print [term] -   print [term]
save [file]     -   save the settlement to file
load [file]     -   load a settlement from a file
"#;

pub mod filedisp {
    use std::process::Command;

    /// Display the help file in a child process.
    /// Will fail if help.txt is moved, renamed or deleted.
    pub fn help() {
        let mut output = if cfg!(target_os = "windows") {
            Command::new("notepad")
                .arg("docs\\help.txt")
                .spawn().unwrap_or_else(|e| {
                    panic!("Failed to execute process: {}", e)
                })
        } else {
            Command::new("less")
                .arg("docs/HELP")
                .spawn().unwrap_or_else(|e| {
                    panic!("Failed to execute process: {}", e)
                })
        };

        output.wait().expect("Failed to wait on help process");
    }

    /// Display the license file
    /// Will fail if LICENSE is moved, renamed or deleted.
    pub fn license() {
        let mut output = if cfg!(target_os = "windows") {
            Command::new("notepad")
                .arg("LICENSE")
                .spawn().unwrap_or_else(|e| {
                    panic!("Failed to execute process: {}", e)
                })
        } else {
            Command::new("less")
                .arg("LICENSE")
                .spawn().unwrap_or_else(|e| {
                    panic!("Failed to execute process: {}", e)
                })
        };

        output.wait().expect("Failed to wait on help process");
    }
}
