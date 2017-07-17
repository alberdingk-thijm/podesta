#![allow(dead_code)]
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub entries: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub date: String,
    pub info: String,
}

impl History {
    /// Create a new History with no entries.
    pub fn new() -> History {
        History { entries: vec!() }
    }

    /// Add an entry to the history with the given date and information.
    pub fn add_entry(&mut self, date: String, info: String) {
        self.entries.push(Entry::new(date, info))
    }

    /// Return a vector of entries matching the given date.
    pub fn get_date(&self, date: String) -> Vec<&Entry> {
        self.entries.iter().filter(|x| x.date == date).collect::<Vec<_>>()
    }

    /// Return a string representation of the entries in the history.
    /// If a date is given, print only the entries matching that date.
    pub fn show(&self, date: Option<String>) -> String {
        match date {
            Some(d) => self.get_date(d),
            None => self.entries.iter().collect::<Vec<_>>(),
        }.iter().map(|e| format!("{}\n", e.to_string())).collect::<String>()
    }
}

impl Entry {
    fn new(date: String, info: String) -> Entry {
        Entry { date: date, info: info }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.date, self.info)
    }
}
