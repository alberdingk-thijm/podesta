//use buildings;
use people;
//use effects;
use sett;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
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

impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The {} Quarter, with a {} focus. {} steps old. \
               Population {}, mostly {}.",
               self.name,
               self.qtype,
               self.age,
               self.pop,
               self.race)
    }
}

macro_attr! {
    #[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq,
             IterVariants!(QTypeVariants),
             IterVariantNames!(QTypeVariantNames),
             EnumDisplay!, EnumFromStr!)]
    /// The focus of the quarter.
    pub enum QType {
        /// Mostly residences
        Residential,
        /// Mostly artisans and shops
        Industrial,
        /// Waterfront buildings
        Port,
        /// Educational institutions
        Academic,
        /// Military and government buildings
        Administrative,
    }
}

#[derive(Debug)]
/// Possible errors when trying to construct a new quarter or building
pub enum BuildError {
    /// Not enough population to construct a quarter
    NotEnoughPop,
    /// Not enough gold to construct a building
    NotEnoughGold,
    /// A building of that name already exists
    AlreadyExists,
    /// Port quarter can't be constructed inland
    InlandPort,
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
