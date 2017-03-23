use quarters;

#[derive(Serialize, Deserialize, Debug)]
pub struct Building {
    pub name: String,
    pub id: i32,
    #[serde(rename = "type")]
    pub btype: quarters::QType,  // use serde macro to match type
    pub preq: Option<Vec<String>>,  // can be null
    pub cost: f32,
    pub build: f32,
    pub events: Vec<EventChance>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct EventChance {
    name: String,
    chance: f32,
}

impl Building {

}
