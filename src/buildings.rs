use serde;

#[derive(Serialize, Deserialize, Debug)]
pub struct Building {
    pub name: String,
    pub id: i32,
    #[serde(rename = "type")]
    pub btype: BType,  // use serde macro to match type
    pub preq: Option<Vec<String>>,  // can be null
    pub cost: f32,
    pub build: f32,
    pub events: Vec<EventChance>,
    pub flags: i32,
}

#[derive(Serialize, Deserialize, Debug)]
enum BType {
    Residential,
    Industrial,
    Port,
    Academic,
    Administrative,
}

impl fmt::Display for BType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, match *self {
            BType::Residential => "RESIDENTIAL",
            BType::Industrial => "INDUSTRIAL",
            BType::Port => "PORT",
            BType::Academic => "ACADEMIC",
            BType::Administrative => "ADMINISTRATIVE")
        }
    }
}

impl str::FromStr for BType {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RESIDENTIAL" => Ok(BType::Residential),
            "INDUSTRIAL" => Ok(BType::Industrial),
            "PORT" => Ok(BType::Port),
            "ACADEMIC" => Ok(BType::Academic),
            "ADMINISTRATIVE" => Ok(BType::Administrative),
            _ => Err(()),
        }
    }

impl ::serde::Serialize for BType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where S: ::serde::Serializer {
        // serialize enum as string in all caps
        let name = *self.to_string();
        serializer.serialize_str(name)
    }
}

impl ::serde::Deserialize for BType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where D: ::serde::Deserializer {
        
        struct Visitor;

        impl 
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct EventChance {
    name: String,
    chance: f32,
}

impl Building {

}
