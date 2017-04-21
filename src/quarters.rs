//use buildings;
use people;
//use effects;
use sett;
use std::fmt;
use std::error;

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
        /// Waterfront buildings (only available in coastal settlements)
        Port,
        /// Educational institutions
        Academic,
        /// Military and government buildings
        Administrative,
    }
}

impl QType {
    /// Return a vec of QTypes, filtering out the Port QType if
    /// is_coastal is false.
    pub fn get_qtypes(is_coastal: bool) -> Vec<Self> {
        Self::iter_variants().filter(|&q| is_coastal || q != QType::Port)
            .collect::<Vec<_>>()
    }

    /// Return a vec of QType strings, filtering out the Port QType if
    /// is_coastal is false.
    pub fn get_qtype_names(is_coastal: bool) -> Vec<String> {
        Self::iter_variant_names()
            .filter(|&q| is_coastal || q != format!("{}", QType::Port))
            .map(|x| x.to_string()).collect::<Vec<_>>()
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
impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BuildError::NotEnoughPop =>
                write!(f, "Population not high enought to build"),
            BuildError::NotEnoughGold =>
                write!(f, "Gold not high enough to build"),
            BuildError::AlreadyExists =>
                write!(f, "A structure by that name already exists"),
            BuildError::InlandPort =>
                write!(f, "Cannot add a port quarter to an inland sett"),
        }
    }
}

impl error::Error for BuildError {
    fn description(&self) -> &str {
        match *self {
            BuildError::NotEnoughPop => "not enough population",
            BuildError::NotEnoughGold => "not enough gold",
            BuildError::AlreadyExists => "reused unique name",
            BuildError::InlandPort => "inland port",
        }
    }

    fn cause(&self) -> Option<&error::Error> { None }
}


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
    pub fn step(&mut self, growth: i32) {
        self.age += 1;
        self.pop += growth;
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
