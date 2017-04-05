use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Region {
    pub name: String,
    pub desc: String,
    pub growth: i32,
    pub starting_gold: f64,
}

impl Region {

}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.name, self.desc)
    }
}
