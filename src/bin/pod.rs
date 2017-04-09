extern crate libc;
extern crate serde_json;
extern crate serde;
extern crate rouler;

extern crate podesta;

use std::io::{self, Write};
//use std::rc::Rc;

fn main() {
    let data = podesta::DataFiles::new("regions.json", "buildings.json",
                                       "events.json");
    let mut automate = false;
    // Display the welcome message
    println!("{}", podesta::WELCOME_MINI);
    // Prompt for a new settlement
    let mut sett = podesta::new_sett(&data, automate);


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
            ParseResult::Step => sett.step(), // make sure initialized
            ParseResult::New => sett = podesta::new_sett(&data, automate),
            ParseResult::ToggleAuto => {
                automate = !automate;
                println!("automate = {}", automate);
            },
            ParseResult::Print(s) => println!("{}", s),
            ParseResult::Quit => break,
        }
    }
}

enum ParseResult {
    Success,
    Step,
    New,
    Print(String),
    ToggleAuto,
    Quit,
}

fn parse_input(input: &str) -> ParseResult {
    match input {
        "help" => podesta::filedisp::help(),
        "license" => podesta::filedisp::license(),
        "commands" => return ParseResult::Print(podesta::COMMANDS.to_string()),
        "new" => return ParseResult::New,
        "step" | "n" | "next" => return ParseResult::Step,
        "p" | "print" => (),
        "a" | "auto" => return ParseResult::ToggleAuto,
        "q" | "quit" => return ParseResult::Quit,
        "" => (),
        _ => return ParseResult::Print(format!("Unknown option \"{}\"", input)),
    }
    ParseResult::Success
}
