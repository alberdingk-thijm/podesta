/// Functionality for prompting the user to make a choice or type some text.

use std::io::{self, Write};
use std::num;
use rand::{self, Rng};
use std::fmt;

#[derive(Debug)]
pub enum PromptError {
    Io(io::Error),
    Parse(num::ParseIntError),
    InvalidNum,
    NameTooShort,
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
