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
        use podesta::interpreter::ParseResult as ParseResult;
        // Generate the settlement
        // Ask for user input
        print!("> ");
        io::stdout().flush()
            .expect("Failed to flush stdout!");

        input.clear();
        io::stdin().read_line(&mut input)
            .expect("Error reading input!");
        let input = input.trim();
        match podesta::interpreter::parse_input(input) {
            ParseResult::Success => (),
            ParseResult::DispFile(app, file) => {
                use std::process::Command;
                let mut output = Command::new(app)
                    .arg(file)
                    .spawn().unwrap_or_else(|e| {
                        panic!("Failed to execute process: {}", e)
                    });
                output.wait().expect("Failed to wait on process");
            },
            ParseResult::Step(n) =>
                match sett {
                    // Make sure a sett exists
                    Some(ref mut s) => for _ in 1..n { s.step() },
                    None => println!("No settlement found \
                                     (first run 'new' or 'load')!"),
            },
            ParseResult::New(_) => {
                if sett.is_some() {
                    sett = podesta::new_sett_confirm(&data, automate)
                        .or(sett);
                } else {
                    sett = Some(podesta::new_sett(&data, automate));
                }
            },
            ParseResult::ToggleAuto => {
                automate = !automate;
                println!("Automation set to {}", automate);
            },
            ParseResult::Save(_) => (), //podesta::save(sett, format!("{}.rgs", sett.name)),
            ParseResult::Load(file) => println!("{:?}", podesta::load(file)),
            ParseResult::Print(s) => println!("{}", s),
            ParseResult::Quit => break,
        }
    }
}
