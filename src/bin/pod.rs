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
    // Allow the program to start without a settlement
    let mut sett : Option<podesta::sett::Sett> = None;

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
            ParseResult::Step(n) => match sett {
                // Make sure a sett exists
                Some(ref mut s) => for _ in 1..n { s.step() },
                None => println!("No settlement found \
                                 (first run 'new' or 'load')!"),
            },
            ParseResult::New => sett = Some(podesta::new_sett(&data, automate)),
            ParseResult::ToggleAuto => {
                automate = !automate;
                println!("Automation set to {}", automate);
            },
            ParseResult::Save => (), //podesta::save(sett, format!("{}.rgs", sett.name)),
            ParseResult::Load(file) => println!("{:?}", podesta::load(file)),
            ParseResult::Print(s) => println!("{}", s),
            ParseResult::Quit => break,
        }
    }
}

enum ParseResult {
    Success,
    Step(i64),
    New,
    Print(String),
    Save,
    Load(String),
    ToggleAuto,
    Quit,
}

fn parse_input(input: &str) -> ParseResult {
    match input {
        "help" => podesta::filedisp::help(),
        "license" => podesta::filedisp::license(),
        "commands" => return ParseResult::Print(podesta::COMMANDS.to_string()),
        "new" => return ParseResult::New,
        "step" | "n" | "next" => return ParseResult::Step(1),
            //TODO: add # of steps opt
        "p" | "print" => (),
        "a" | "auto" => return ParseResult::ToggleAuto,
        "q" | "quit" => return ParseResult::Quit,
        "save" => return ParseResult::Save,
        "load" => return ParseResult::Load("foo".to_string()),
        "" => (),
        _ => return ParseResult::Print(format!("Unknown option \"{}\"", input)),
    }
    ParseResult::Success
}
