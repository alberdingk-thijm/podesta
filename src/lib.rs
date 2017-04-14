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
pub mod events;
pub mod effects;
pub mod interpreter;

use std::rc::Rc;
use std::io;

/// Return Some(a new settlement) but only after prompting for confirmation.
/// If confirmation is not received, return None.
/// See new_sett.
pub fn new_sett_confirm(data: &parser::DataFiles, auto: bool) -> Option<sett::Sett> {
    if prompts::bool_choose("Overwrite existing settlement? (y/n): ",
        &["y", "yes"], &["n", "no"]).unwrap_or(false) {
        Some(new_sett(&data, auto))
    } else {
        None
    }
}

/// Create a new settlement, prompting for user input occasionally.
pub fn new_sett(data: &parser::DataFiles, auto: bool) -> sett::Sett {
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

    // Coastal
    println!("Choos{} if {} is coastal...",
             if auto { "ing" } else { "e" }, name);
    let coastchoice = prompts::bool_choose_or_rand(
        &format!("Is {} coastal? (y/n): ", name),
        &["y", "yes"], &["n", "no"], numprompts);

    // Quarter type
    println!("Choos{} the focus of {}'s main quarter...",
             if auto { "ing" } else { "e" }, name);
    // If the settlement is not coastal, remove the Port option
    // i.e. if coastal or qtype not Port, keep it
    let qtypenames = quarters::QType::iter_variant_names()
        .filter(|&q| coastchoice || q != format!("{}", quarters::QType::Port));
    let qchoice = prompts::choose_or_rand(
        &(qtypenames.collect::<Vec<_>>()), numprompts);
    let mut qtypes = quarters::QType::iter_variants()
        .filter(|&q| coastchoice || q != quarters::QType::Port);
    let qtype = qtypes.nth(qchoice).unwrap();

    // Race
    println!("Choos{} the majority race of {}'s main quarter...",
             if auto { "ing" } else { "e" }, name);
    let racenames = people::Race::iter_variant_names();
    let racechoice = prompts::choose_or_rand(
        &(racenames.collect::<Vec<_>>()), numprompts);
    let race = people::Race::iter_variants().nth(racechoice).unwrap();

    // Generate empty settlement
    let s = sett::Sett::new(name,
                            reg,
                            qtype,
                            race,
                            coastchoice);
    println!("Generated a settlement:\n{}", s);
    s
}

//TODO: how to know what datafiles were used?
/// Save the settlement and its associated information to the given file name.
pub fn save(sett: sett::Sett, fname: String) -> io::Result<String> {
    Ok(format!("Saved {} to {}!", sett.name, fname))
}

/// Load the settlement and its associated information from the given file name.
pub fn load(fname: String) -> io::Result<sett::Sett> {
    unimplemented!()
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
