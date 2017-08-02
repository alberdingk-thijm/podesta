/// Functionality for interpreting user commands.

use std::str;
use shlex;

/// List of possible user commands
pub enum ParseResult {
    /// Succeed without modifying state.
    Success,
    /// Step some number of times.
    Step(i64),
    /// Create a new object based on the vector.
    New(Option<String>, Option<String>, Option<String>),
    /// Print the named object to the screen.
    Print(Option<String>, Option<String>, Option<String>),
    /// Save the environment to a file.
    Save(Option<String>),
    /// Load a file into the environment.
    Load(Option<String>),
    /// Toggle user prompting.
    ToggleAuto,
    /// Toggle dev mode.
    ToggleDev,
    /// Print list of available commands to the screen.
    Commands,
    /// Display a file using the specified program.
    DispFile(String, String),
    /// Display an unknown option message.
    Unknown(String),
    /// Quit the application.
    Quit,
}

pub fn parse_input(input: &str) -> ParseResult {
    let mut cmd = shlex::Shlex::new(input);
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
            "commands" => ParseResult::Commands,
            "new" | "add" => ParseResult::New(cmd.next(), cmd.next(), cmd.next()),
            "step" | "n" | "next" =>
                ParseResult::Step(cmd.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(1)),
            "p" | "print" => ParseResult::Print(cmd.next(), cmd.next(), cmd.next()),
            "a" | "auto" => ParseResult::ToggleAuto,
            "q" | "quit" => ParseResult::Quit,
            "sv" | "save" => ParseResult::Save(cmd.next()),
            "ld" | "load" => ParseResult::Load(cmd.next()),
            "dev" => ParseResult::ToggleDev, //TODO: add dev mode
                                           //TODO: allow normally blocked commands
                                           //TODO: e.g. add building w/o matching quarter or gold,
                                           //TODO: create a hero or perform an event's effects
            "" => ParseResult::Success,
            s @ _ => ParseResult::Unknown(s.to_string()),
        },
        None => ParseResult::Success,
    }
}
