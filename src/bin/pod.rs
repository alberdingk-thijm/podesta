extern crate libc;
extern crate serde_json;
extern crate serde;
extern crate rouler;

extern crate podesta;

use podesta::manager::Manager as Manager;
use podesta::libdata::PathList as PathList;

use std::io::{self, Write};

pub const DATA_DIR: &'static str = "lib/data/";
pub const NAME_DIR: &'static str = "lib/names/";

fn main() {
    // Display the welcome message
    println!("{}", podesta::WELCOME_MINI);
    // Initialize the manager
    let pl = PathList::from_dirs(DATA_DIR, NAME_DIR)
        .expect("Invalid lib dirs!");
    let mut man = Manager::new(&pl, true);
    let mut input = String::new();
    'game: loop {
        use podesta::interpreter::ParseResult as ParseResult;
        print!("> ");
        io::stdout().flush()
            .expect("Failed to flush stdout!");
        input.clear();
        io::stdin().read_line(&mut input)
            .expect("Error reading input!");
        let input = input.trim();
        // Separate separate terms into chunks
        // TODO: clean out arrow key presses and other key codes
        // codes: ESC[A, ESC[B, ESC[C, ESC[D
        let terms = input.split(';').map(|s| s.trim());
        //dev: check state of event queue
        //let print_eq = [String::from("p queue")].iter();
        //terms = terms.chain(print_eq);
        'terms: for term in terms {
            //println!("{:?}", term);
            match podesta::interpreter::parse_input(term) {
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
                ParseResult::New(target, name, area) => {
                    match target {
                        Some(t) => match t.as_str() {
                            "sett" => man.build_sett(name, false),
                            "quarter" => man.build_quarter(name),
                            "building" => man.build_building(name, area),
                            _ => println!("Invalid target for 'new' \
                                          (specify 'sett', 'quarter' \
                                           or 'building')!"),
                        },
                        None => println!("Invalid target for 'new' \
                                         (specify 'sett', 'quarter' \
                                          or 'building')!"),
                    }
                },
                ParseResult::Repair(bname, qname) => man.repair_building(bname, qname),
                ParseResult::ToggleAuto => man.toggle_auto(),
                ParseResult::ToggleDev => man.toggle_dev(),
                ParseResult::Commands => println!("{}", podesta::COMMANDS),
                ParseResult::Save(file) => man.save(file).unwrap_or_else(|e| {
                    println!("Failed to save the game file! {:?}", e);
                }),
                ParseResult::Load(file) => match Manager::load(file) {
                    Ok(m) => { man = m; println!("Loaded {}!", man.get_savefile()) },
                    Err(e) => println!("Failed to load the game file! {:?}",
                                       e),
                },
                ParseResult::Print(s1, s2, s3) => man.print(s1, s2, s3),
                ParseResult::Unknown(s) => println!("Unknown option \"{}\"", s),
                ParseResult::Quit => break 'game,
            }
        }
    }
}
