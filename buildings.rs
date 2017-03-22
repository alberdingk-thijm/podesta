

#[derive(Serialize, Deserialize, Debug)]
struct Building {
    name: String,
    id: i32,
    btype: BType,  // use serde macro to match type
    preq: Option<Vec<String>>,  // can be null
    cost: f32,
    build: f32,
    events: Vec<EventChance>,
    flags: i32,
}

#[derive(Serialize, Deserialize, Debug)]
enum BType {
    Residential,
    Industrial,
    Port,
    Academic,
    Administrative,
}

#[derive(Serialize, Deserialize, Debug)]
struct EventChance {
    name: String,
    chance: f32,
}

impl Building {

}
