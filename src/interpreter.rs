#![allow(dead_code)]
/// Functionality for interpreting user commands.

use std::str;
use COMMANDS;

#[derive(Debug)]
struct Command<'a> {
    parse: CommandParse<'a>,
}

#[derive(Debug)]
struct CommandParse<'a> {
    terms: &'a str,
    pos: usize,
}

impl<'a> CommandParse<'a> {
    fn new(s: &str) -> CommandParse {
        CommandParse { terms: s, pos: 0 }
    }

    /*
    /// Consume the CommandParse and produce a hash map based on the terms
    /// Order of terms: (terms in square brackets are optional)
    /// * new sett [name] [-r region] [-c] [-q type] [-p race]
    /// * new quarter [name] [-q type] [-p race] in [homeS]
    /// * new building [name] in [homeQ of homeS]
    /// * print sett name
    /// * print quarter name in homeS
    /// * print building name in homeQ of homeS
    fn to_hashmap(self) -> HashMap {
        let mut hashmap = HashMap::New();
        let action = self.next().0;
        let target = self.next().0;
        for term in self {
            match term {
                Some(s) => match s.as_str() {
                    ("-r", reg) if target == "sett" => {
                        // Next term is a region
                        hashmap.insert(CommandPart::Region, self.next());
                    },
                    ("-c") if target == "sett" => {
                        // Mark coast true
                        hashmap.insert(CommandPart::Coastal, s)
                    },
                    ("-q", qtype) if target == "sett" || target == "quarter" => {
                        // Next term is quarter type
                        hashmap.insert(CommandPart::QType, self.next());
                    },
                    ("-p", race) if target == "sett" || target == "quarter" => {
                        // Next term is race
                        hashmap.insert(CommandPart::Race, self.next());
                    },
                    ("in", homes) => (),
                    ("in", homeq, "of", homes) => ()
                },
                None => (),
            }
        }
    }
    */
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum CommandPart<'a, 'b> {
    Action(&'a str),
    Target(&'a str),
    Bool(&'a str),
    KeyVal(&'a str, &'b str),
    Param(&'a str),
}

impl<'a> Iterator for CommandParse<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut term = String::new();
        let mut in_quotes = false;
        let mut pos = 0;
        let next_term = self.terms.chars().skip(self.pos);
        for c in next_term {
            pos += 1;
            match c {
                '"' => {
                    // flip value of in_quotes (since quotes come in pairs)
                    in_quotes = !in_quotes;
                },
                _ if c.is_whitespace() && !in_quotes => {
                    // end current term or arg unless in quotes
                    // TODO: increase pos to just before next char
                    break;
                },
                _ => {
                    // add to current term
                    term.push(c);
                },
            }
        }
        if in_quotes || pos == 0 {
            None
        } else {
            self.pos += pos;
            Some(term)
        }
    }
}

/// List of possible user commands
pub enum ParseResult {
    /// Succeed without modifying state.
    Success,
    /// Step some number of times.
    Step(i64),
    /// Create a new object based on the vector.
    New(Option<String>, Option<String>),
    /// Print the given string to the screen.
    Print(String),
    /// Save the environment to a file.
    Save(Option<String>),
    /// Load a file into the environment.
    Load(String),
    /// Toggle user prompting.
    ToggleAuto,
    /// Display a file using the specified program.
    DispFile(String, String),
    /// Quit the application.
    Quit,
}

pub fn parse_input(input: &str) -> ParseResult {
    let mut cmd = CommandParse::new(input);
    let action = cmd.next();
    match action {
        Some(s) => match s.as_str() {
            "help" => if cfg!(target_os = "windows") {
                ParseResult::DispFile("notepad".to_string(),
                    "docs\\help.txt".to_string())
            } else {
                ParseResult::DispFile("less".to_string(),
                    "docs/HELP".to_string())
            },
            "license" => if cfg!(target_os = "windows") {
                ParseResult::DispFile("notepad".to_string(),
                    "LICENSE".to_string())
            } else {
                ParseResult::DispFile("less".to_string(),
                    "LICENSE".to_string())
            },
            "commands" => ParseResult::Print(COMMANDS.to_string()),
            "new" => ParseResult::New(cmd.next(), cmd.next()),
            "step" | "n" | "next" => ParseResult::Step(cmd.next().map(|s| s.parse::<i64>().unwrap_or(1)).unwrap_or(1)),
            "p" | "print" => ParseResult::Success, //ParseResult::Print(cmd.collect::Vec<_>()),
            "a" | "auto" => ParseResult::ToggleAuto,
            "q" | "quit" => ParseResult::Quit,
            "sv" | "save" => ParseResult::Save(cmd.next()),
            "ld" | "load" => ParseResult::Load(cmd.next().unwrap()),
            "" => ParseResult::Success,
            _ => ParseResult::Print(format!("Unknown option \"{}\"", s)),
        },
        None => ParseResult::Success,
    }
}
