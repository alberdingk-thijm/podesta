use std::fmt;
use std::str;
use buildings;
use people;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Quarter {
    pub name: String,
    pub qtype: QType,
    pub pop: i32,
    pub age: i32,
    pub race: people::Race,
    pub bldgs: Vec<buildings::Building>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum QType {
    Residential,
    Industrial,
    Port,
    Academic,
    Administrative,
}

impl fmt::Display for QType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            QType::Residential => "Residential",
            QType::Industrial => "Industrial",
            QType::Port => "Port",
            QType::Academic => "Academic",
            QType::Administrative => "Administrative",
        })
    }
}

impl str::FromStr for QType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Residential" => Ok(QType::Residential),
            "Industrial" => Ok(QType::Industrial),
            "Port" => Ok(QType::Port),
            "Academic" => Ok(QType::Academic),
            "Administrative" => Ok(QType::Administrative),
            _ => Err(()),
        }
    }
}

impl Quarter {
    /// Execute timestep
    pub fn step(&self) {
        unimplemented!()
    }

    /// Add a building
    pub fn construct(&self, bname: &str) -> Result<Self, Err> {
        unimplemented!()
    }
}