extern crate libc;
extern crate serde_json;
extern crate serde;
extern crate rouler;

extern crate podesta;

use std::io::{self, Write};

fn main() {

    /*
    let events : Vec<podesta::events::Event> = podesta::parser::get_data("events.json")
        .expect("Error parsing JSON!");

    println!("Loaded events.json! {} events found", events.len());
    */

    let plans : Vec<podesta::buildings::BuildingPlan> = podesta::parser::get_data("buildings.json")
        .expect("Error parsing JSON!");

    println!("Loaded buildings.json! {} buildings found", plans.len());

    let regions : Vec<podesta::regions::Region> = podesta::parser::get_data("regions.json")
        .expect("Error parsing JSON!");

    println!("Loaded regions.json! {} regions found", regions.len());

    // Display the welcome message
    println!("{}", podesta::WELCOME_MINI);
    let mut input = String::new();
    loop {
        // Generate the settlement
        // Ask for user input
        print!("> ");
        io::stdout().flush()
            .expect("Failed to flush stdout!");

        input.clear();
        io::stdin().read_line(&mut input)
            .expect("Error reading input!");
        let input = input.trim();
        match parse_input(input) {
            ParseResult::Success => (),
            ParseResult::Error(s) => println!("{}", s),
            ParseResult::Quit => break,
        }
    }
}

enum ParseResult {
    Success,
    Error(String),
    Quit,
}

fn parse_input(input: &str) -> ParseResult {
    match input {
        "help" => podesta::help(),
        "license" => podesta::license(),
        "commands" => println!("{}", podesta::COMMANDS),
        "new" => podesta::init(),
        "q" | "quit" => return ParseResult::Quit,
        _ => return ParseResult::Error(format!("Unknown option \"{}\"", input)),
    }
    ParseResult::Success
}
