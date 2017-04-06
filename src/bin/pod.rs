extern crate libc;
extern crate serde_json;
extern crate serde;
extern crate rouler;

extern crate podesta;

use std::io::{self, Write};
//use std::rc::Rc;

fn main() {
    /*
    println!("Loaded events.json! {} events found", events.len());
    */
    let data = podesta::DataFiles::new("regions.json", "buildings.json");
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
            ParseResult::New => podesta::init(&data), //.clone()),
            ParseResult::Info(s) => println!("{}", s),
            ParseResult::Quit => break,
        }
    }
}

enum ParseResult {
    Success,
    New,
    Info(String),
    Quit,
}

fn parse_input(input: &str) -> ParseResult {
    match input {
        "help" => podesta::help(),
        "license" => podesta::license(),
        "commands" => return ParseResult::Info(podesta::COMMANDS.to_string()),
        "new" => return ParseResult::New,
        "print" => (),
        "q" | "quit" => return ParseResult::Quit,
        "" => (),
        _ => return ParseResult::Info(format!("Unknown option \"{}\"", input)),
    }
    ParseResult::Success
}
