//use buildings;
use people;
//use effects;
use sett;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Quarter {
    /// The unique name of the quarter.
    pub name: String,
    /// The quarter's "type" (what kind of activities take place here).
    pub qtype: QType,
    /// The total population of the quarter.
    pub pop: i32,
    /// The total age in steps of the quarter.
    pub age: i32,
    /// The majority race of the quarter.
    pub race: people::Race,
    // The buildings constructed in the quarter.
    //pub bldgs: Vec<buildings::Building>,
}

impl sett::HasName for Quarter {
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum QType {
    Residential,
    Industrial,
    Port,
    Academic,
    Administrative,
}

#[derive(Debug)]
pub enum BuildError {
    NotEnoughPop,
    NotEnoughGold,
    AlreadyExists,
}

/*
impl error::Error for BuildError {
    fn description(&self) -> &str {
        match *self {
            BuildError::NotEnoughPop => "Population not high enough to build",
            BuildError::NotEnoughGold => "Gold not high enough to build",
            BuildError::AlreadyExists => "Building already exists!",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
        }
    }
}
*/

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
            //bldgs: vec!(),
        }
    }

    /// Execute timestep
    pub fn step(&self) {
        unimplemented!()
    }

    /*
    /// Add a building
    pub fn add_building(&self, bname: &str) -> Result<Self, BuildError> {
        unimplemented!()
    }

    /// Find a building
    pub fn find_building(&self, bname: &str) -> Option<&buildings::Building> {
        unimplemented!()
    }
    */
}
