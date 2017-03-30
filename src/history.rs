
pub struct History {
    pub entries: Vec<Entry>,
}

pub struct Entry {
    pub date: String,
    pub info: String,
}

impl History {
    fn new() -> History {
        History { entries: vec!() }
    }

    fn add_entry(&mut self, date: String, info: String) {
        self.entries.push(Entry::new(date, info))
    }

    fn get_date(&self, date: String) -> &[Entry] {
        self.entries.into_iter().filter(|x| x.date == date).collect()
    }
}

impl Entry {
    fn new(date: String, info: String) -> Entry {
        Entry { date: date, info: info }
    }
}
