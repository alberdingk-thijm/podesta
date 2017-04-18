#![allow(dead_code)]

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
    pub fn new() -> History {
        History { entries: vec!() }
    }

    pub fn add_entry(&mut self, date: String, info: String) {
        self.entries.push(Entry::new(date, info))
    }

    pub fn get_date(&self, date: String) -> Vec<&Entry> {
        self.entries.iter().filter(|x| x.date == date).collect::<Vec<_>>()
    }
}

impl Entry {
    fn new(date: String, info: String) -> Entry {
        Entry { date: date, info: info }
    }
}
