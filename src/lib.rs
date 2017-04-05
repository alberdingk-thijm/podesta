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

use std::fmt;
use rand::Rng;
use std::io::{self, Write};
use std::num;
use std::rc::Rc;
//use std::error;

pub struct DataFiles {
    regions: Rc<Vec<regions::Region>>,
    plans: Rc<Vec<buildings::BuildingPlan>>,
    //events: Vec<events::Event>,
}

impl DataFiles {
    pub fn new(building_path: &str, region_path: &str) -> DataFiles {
        DataFiles {
            regions: Rc::new(parser::get_data(region_path)
                .and_then(|d| {
                    println!("Loaded {}! {} regions found.",
                                       region_path, d.len());
                    Ok(d)
                }).expect("Error parsing regions JSON!")),
            plans: Rc::new(parser::get_data(building_path)
                .and_then(|d| {
                    println!("Loaded {}! {} buildings found.",
                                       building_path, d.len());
                    Ok(d)
                }).expect("Error parsing buildings JSON!")),
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
pub fn init(data: Rc<DataFiles>) {
    // Prompt for region (max 2 tries)
    let regchoice = prompt_or_rand(&(data.regions), 2);
    let reg = data.regions
        //TODO: cannot move elements inside data.regions (wrap with Box?)
        .into_iter().nth(regchoice).expect("Unable to get region!");
    // Prompt for quarter type
    let qtype = quarters::QType::Port;
    // Prompt for race?
    let race = people::Race::Human;
    // Generate empty settlement
    let mut s = sett::Sett::new("Amsterdam",
                                Box::new(reg),
                                qtype,
                                race,
                                sett::SettFlags::Coastal);
    println!("Generated a settlement: {:?}", s);

    println!("{:?}", s.add_quarter("Jewish Quarter".to_string(),
                  quarters::QType::Industrial,
                  people::Race::Human));

    println!("Settlement state: {:?}", s);
}

enum PromptError {
    Io(io::Error),
    Parse(num::ParseIntError),
    InvalidNum,
}
/// Prompt the user for a choice from the given list of displayable items.
/// Return Ok(item index) if the chosen **item** was successfully picked,
/// otherwise return Err(PromptError).
fn prompt_choice<T: fmt::Display>(a: &[T]) -> Result<usize, PromptError> {
    // Remember to decrement the choice by 1 to get the actual index
    for choice in 1..a.len() {
        println!("{}: {}", choice, a[choice-1]);
    }
    print!("Choose a number from 1 to {}: ", a.len());
    io::stdout().flush()
        .expect("Failed to flush to stdout!");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().parse::<usize>().map(|x| x - 1)
            .map_err(PromptError::Parse),
        Err(e) => Err(PromptError::Io(e)),
    }.and_then(|x| if x > 0 && x < a.len() {
        Ok(x)
    } else {
        Err(PromptError::InvalidNum)
    })
}

/// Prompt the user for a choice from the given list of displayable items,
/// and return the index of the chosen item. If the user fails to make a
/// choice maxprompts times, pick a random item index.
fn prompt_or_rand<T: fmt::Display>(a: &[T], maxprompts: i32) -> usize {
    let mut numprompts = 0;
    while numprompts < maxprompts {
        match prompt_choice(a) {
            Ok(c) => return c,
            Err(PromptError::Io(e)) => println!("{}", e),
            Err(_) => println!("Please choose a valid number."),
        }
        numprompts += 1;
    }
    rand::thread_rng().gen_range(0, a.len())
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
