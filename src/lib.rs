extern crate libc;
#[macro_use] extern crate serde_derive;
extern crate bincode;
extern crate serde_json;
extern crate serde;
extern crate rouler;
extern crate rand;
#[macro_use] extern crate macro_attr;
#[macro_use] extern crate enum_derive;
extern crate time;

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
pub mod history;
pub mod manager;


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
sv, save [file] -   save the settlement to file
ld, load [file] -   load a settlement from a file
"#;
