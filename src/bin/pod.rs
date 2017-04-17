extern crate libc;
extern crate serde_json;
extern crate serde;
extern crate rouler;

extern crate podesta;

use std::io::{self, Write};
//use std::rc::Rc;

fn main() {
    let data = podesta::parser::DataFiles::new("regions.json", "buildings.json",
                                               "events.json");
    let mut automate = false;
    // Display the welcome message
    println!("{}", podesta::WELCOME_MINI);
    let mut man = podesta::manager::Manager::new();
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
            ParseResult::New(target, _) => {
                // TODO: rewrite new_sett to new and take a target and name
                match target {
                    Some(t) => match t.as_str() {
                        "sett" => {
                            if sett.is_some() {
                                sett = podesta::new_sett_confirm(&data, automate)
                                    .or(sett);
                            } else {
                                sett = Some(podesta::new_sett(&data, automate));
                            }
                        },
                        "quarter" => {
                            match sett {
                                Some(_) => {
                                    //s.add_quarter()
                                },
                                None => println!("No settlement found \
                                              (first run 'new' or 'load')!"),
                            }
                        },
                        "building" => {
                            match sett {
                                Some(_) => {
                                    // get q
                                    // q.add_building()
                                },
                                None => println!("No settlement found \
                                              (first run 'new' or 'load')!"),
                            }
                        },
                        _ => println!("Invalid target for 'new' \
                                      (specify 'sett', 'quarter' or 'building')!"),
                    },
                    None => println!("Invalid target for 'new' \
                                     (specify 'sett', 'quarter' or 'building')!"),
                }
            },
            ParseResult::ToggleAuto => {
                automate = !automate;
                println!("Automation set to {}", automate);
            },
            ParseResult::Save(file) => {
                match sett {
                    Some(ref s) => {
                        podesta::parser::save_rbs(&man, file
                                                  .unwrap_or(s.name.clone()).as_str())
                            .unwrap_or_else(|e| {
                                println!("Failed to save the game file! {:?}", e);
                            })
                    },
                    None => println!("No settlement found \
                                      (first run 'new' or 'load')!"),
                }
            },
            ParseResult::Load(file) => { podesta::load(file); },
            ParseResult::Print(s) => println!("{}", s),
            ParseResult::Quit => break,
        }
    }
}
