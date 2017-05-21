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
    pub pop: f64,  // Stored as float for added precision, displayed as int
    /// The total age in steps of the quarter.
    pub age: i32,
    /// The majority race of the quarter.
    pub race: people::Race,
    // The buildings constructed in the quarter.
    //pub bldgs: Vec<buildings::Building>,
    /// The growth rate of the quarter's population.
    pub growth: f64,
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
               self.pop as i64,
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
    /// No plan specifies the given building
    NoPlanFound,
    /// No quarter can host the given building
    NoQuarterFound,
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
            BuildError::NoPlanFound =>
                write!(f, "No plan specifies the given building"),
            BuildError::NoQuarterFound =>
                write!(f, "No quarter of the same type as the building exists"),
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
            BuildError::NoPlanFound => "no building plan found",
            BuildError::NoQuarterFound => "no valid quarter found",
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
    pub fn new(n: &str, qt: QType, p: f64, r: people::Race) -> Quarter {
        Quarter {
            name: n.to_string(),
            qtype: qt,
            pop: p,
            age: 0,
            race: r,
            //bldgs: vec!(),
            growth: 0.01,
        }
    }

    /// Execute timestep.
    ///
    /// Growth is calculated using a [logistic function][logif] P(t), expressed
    /// as follows:
    /// `P(t) = cap * start * growth * e^(rt) / cap + start * (e^(rt) - 1)`
    /// where
    /// `P(t)`: quarter population at time t
    /// `t`: time, i.e. the age of the quarter
    /// `cap`: the base carrying capacity of the quarter = 100000
    /// `start`: the base starting population of the quarter = 50
    /// `reg_growth`: the growth modifier of the sett's region
    /// `r`: the growth rate of the quarter
    /// [logif]: https://en.wikipedia.org/wiki/Logistic_function
    pub fn step(&mut self, reg_growth: f64) {
        self.age += 1;
        let grow_rate = |r: f64, t: i32| -> f64 { (r * t as f64).exp() };
        // simplify the constants
        let e_rt = grow_rate(self.growth, self.age);
        self.pop = 5000000.0 * reg_growth * e_rt / (99950.0 + 50.0 * e_rt);
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
