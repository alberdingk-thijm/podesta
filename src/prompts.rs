/// Functionality for prompting the user to make a choice or type some text.

use std::io::{self, Write};
use std::num;
use rand::{self, Rng};
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum PromptError {
    Io(io::Error),
    Parse(num::ParseIntError),
    InvalidNum,
    NameTooShort,
    YesOrNo,
}

impl fmt::Display for PromptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PromptError::Io(ref err) => err.fmt(f),
            PromptError::Parse(ref err) => err.fmt(f),
            PromptError::InvalidNum => write!(f, "number out of bounds"),
            PromptError::NameTooShort => write!(f, "given name too short"),
            PromptError::YesOrNo => write!(f, "answer out of bounds"),
        }
    }
}

impl error::Error for PromptError {
    fn description(&self) -> &str {
        match *self {
            PromptError::Io(ref err) => err.description(),
            PromptError::Parse(ref err) => err.description(),
            PromptError::InvalidNum => "invalid number",
            PromptError::NameTooShort => "name too short",
            PromptError::YesOrNo => "invalid answer",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            PromptError::Io(ref err) => err.cause(),
            PromptError::Parse(ref err) => err.cause(),
            _ => None,
        }
    }
}

/// Prompt the user to type a name with at least minchars length.
/// Return Ok(name) if it was successfully read,
/// otherwise return Err(PromptError).
pub fn name(minchars: usize) -> Result<String, PromptError> {
    print!("Please provide a name: ");
    io::stdout().flush()
        .expect("Failed to flush to stdout!");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim()),
        Err(e) => Err(PromptError::Io(e)),
    }.and_then(|x| if x.len() < minchars {
        Err(PromptError::NameTooShort)
    } else {
        Ok(x.to_string())
    })
}

/// Prompt the user to type a name with at least minchars length.
/// Loop until the name is acceptable, then return it.
pub fn name_loop(minchars: usize) -> String {
    let mut name_input = name(minchars);
    while name_input.is_err() {
        println!("Please enter at least {} letters.", minchars);
        name_input = name(minchars);
    }
    name_input.unwrap()
}

/// Prompt the user for a file name.
pub fn name_file(phrase: &str) -> Result<String, PromptError> {
    print!("Please specify the name of the file{}", phrase);
    io::stdout().flush().expect("Failed to flush to stdout!");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim()),
        Err(e) => Err(PromptError::Io(e)),
    }.and_then(|x| Ok(x.to_string()))
}

/// Prompt the user with a boolean choice, with a given question,
/// expected affirmative answers (returning Ok(true)) and
/// expected negative answers (returning Ok(false)).
/// Behaviour is undefined if the lists of answers overlap.
/// Return Err(PromptError) if it was not successfully read.
pub fn bool_choose(question: &str, aff: &[&str], neg: &[&str])
    -> Result<bool, PromptError>
{
    print!("{}", question);
    io::stdout().flush()
        .expect("Failed to flush to stdout!");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim()),
        Err(e) => Err(PromptError::Io(e)),
    }.and_then(|x| if aff.contains(&x) {
        Ok(true)
    } else if neg.contains(&x) {
        Ok(false)
    } else {
        Err(PromptError::YesOrNo)
    })
}

pub fn bool_choose_or_rand(question: &str, aff: &[&str], neg: &[&str],
                           maxprompts: i32) -> bool {
    let mut numprompts = 0;
    while numprompts < maxprompts {
        match bool_choose(question, aff, neg) {
            Ok(c) => return c,
            Err(PromptError::Io(e)) => println!("{}", e),
            Err(_) => println!("Please make a valid answer."),
        }
        numprompts += 1;
    }
    // coin flip
    rand::thread_rng().gen_range(0, 2) == 0
}

/// Prompt the user for a choice from the given list of displayable items.
/// Return Ok(item index) if the chosen **item** was successfully picked,
/// otherwise return Err(PromptError).
pub fn choose<T: fmt::Display>(a: &[T]) -> Result<usize, PromptError> {
    // Remember to decrement the choice by 1 to get the actual index
    for choice in 1..a.len()+1 {
        println!("{}: {}", choice, a[choice-1]);
    }
    print!("Choose a number from 1 to {}: ", a.len());
    io::stdout().flush()
        .expect("Failed to flush to stdout!");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().parse::<usize>().map(|x| x - 1)
            .map_err(PromptError::Parse),
        Err(e) => Err(PromptError::Io(e)),
    }.and_then(|x| if x < a.len() {
        // Make sure x is within the bounds
        Ok(x)
    } else {
        Err(PromptError::InvalidNum)
    })
}

/// Prompt the user for a choice from the given list of displayable items,
/// and return the index of the chosen item. If the user fails to make a
/// choice maxprompts times, pick a random item index.
pub fn choose_or_rand<T: fmt::Display>(a: &[T], maxprompts: i32) -> usize {
    let mut numprompts = 0;
    while numprompts < maxprompts {
        match choose(a) {
            Ok(c) => return c,
            Err(PromptError::Io(e)) => println!("{}", e),
            Err(_) => println!("Please choose a valid number."),
        }
        numprompts += 1;
    }
    rand::thread_rng().gen_range(0, a.len())
}
