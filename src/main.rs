#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    name: String,
    id: i32,
    desc: String,
    chance: i32,
    event: Vec<String>,
    // TODO: convert to Vec<event effect>
}


fn main() {

    let mut f = try!(File::open("../lib/data/events.json"));
    let mut reader = BufReader::new(f);

    let mut de = serde_json::de::Deserializer<BufReader>::from_reader(reader);
    // convert to Vec<Event>

    let ev = Event {
        name: "FIRE".to_string(),
        id: 1,
        desc: "%s has erupted into flames!".to_string(),
        chance: 3,
        event: vec!("KILL_2".to_string(), "DAM_2".to_string()),
    };

    let serialized = serde_json::to_string(&ev).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Event = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
