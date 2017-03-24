use std::fmt;
use std::str;
use buildings;
use people;

#[derive(Debug)]
pub struct QuarterConf {
    pub name: String,
    pub qtype: QType,
    pub race: people::Race,
}

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

pub type BuildErr = ();  // TODO: convert to actual error
impl Quarter {

    /// Create a new Quarter with a given name, population, type and racial
    /// majority. Age is set to zero and no buildings exist initially.
    pub fn new(n: &str, qt: QType, p: i32, r: people::Race) -> Quarter {
        Quarter {
            name: n.to_string(),
            qtype: qt,
            pop: p,
            age: 0,
            race: r,
            bldgs: vec!(),
        }
    }

    /// Execute timestep
    pub fn step(&self) {
        unimplemented!()
    }

    /// Add a building
    pub fn add_building(&self, bname: &str) -> Result<Self, BuildErr> {
        unimplemented!()
    }
}
