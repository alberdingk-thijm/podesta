/// Functionality for interpreting user commands.

use std::str;
use COMMANDS;

struct Command<'a> {
    terms: &'a str,
    pos: usize,
}

impl<'a> Command<'a> {
    fn new(s: &str) -> Command {
        Command { terms: s, pos: 0 }
    }
}

impl<'a> Iterator for Command<'a> {
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

pub enum ParseResult {
    Success,
    Step(i64),
    New(Vec<String>),
    Print(String),
    Save(Option<String>),
    Load(String),
    ToggleAuto,
    Help,
    License,
    Quit,
}

pub fn parse_input(input: &str) -> ParseResult {
    let mut cmd = Command::new(input);
    let action = cmd.next();
    match action {
        Some(s) => match s.as_str() {
            "help" => ParseResult::Help,
            "license" => ParseResult::License,
            "commands" => ParseResult::Print(COMMANDS.to_string()),
            "new" => ParseResult::New(cmd.collect::<Vec<_>>()),
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
