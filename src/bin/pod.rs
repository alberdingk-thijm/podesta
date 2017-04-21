extern crate libc;
extern crate serde_json;
extern crate serde;
extern crate rouler;

extern crate podesta;

use podesta::manager::Manager as Manager;

use std::io::{self, Write};
//use std::rc::Rc;

fn main() {
    // Display the welcome message
    println!("{}", podesta::WELCOME_MINI);
    // Initialize the manager
    let mut man = Manager::new("regions.json",
                               "buildings.json",
                               "events.json",
                               true);
    let mut input = String::new();
    loop {
        use podesta::interpreter::ParseResult as ParseResult;
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
            ParseResult::Step(n) => man.step(n),
            ParseResult::New(target, name) => {
                // TODO: rewrite new_sett to new and take a target and name
                match target {
                    Some(t) => match t.as_str() {
                        "sett" => man.build_sett(name, false),
                        "quarter" => man.build_quarter(name),
                        "building" => man.build_building(name),
                        _ => println!("Invalid target for 'new' \
                                      (specify 'sett', 'quarter' or 'building')!"),
                    },
                    None => println!("Invalid target for 'new' \
                                     (specify 'sett', 'quarter' or 'building')!"),
                }
            },
            ParseResult::ToggleAuto => man.toggle_auto(),
            ParseResult::Commands => println!("{}", podesta::COMMANDS),
            ParseResult::Save(file) => podesta::parser::save_rbs(
                &man, file.unwrap_or(man.get_sett_name()
                                     .unwrap_or("pod".to_string()))
                .as_str()).unwrap_or_else(|e| {
                    println!("Failed to save the game file! {:?}", e);
            }),
            ParseResult::Load(file) => match Manager::load(file) {
                Ok(m) => man = m,
                Err(e) => println!("Failed to load the game file! {:?}", e),
            },
            ParseResult::Print(s) => man.print(s),
            ParseResult::Unknown(s) => print!("Unknown option \"{}\"", s),
            ParseResult::Quit => break,
        }
    }
}
