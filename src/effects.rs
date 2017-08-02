use rand;
use rand::Rng;
use rouler::Roller;
use quarters::QType;
use items;
use std::str;
use std::default;
use std::ops::{Add, Mul, AddAssign, MulAssign};

/// An enum to determine what part of the settlement the effect should change.
/// There are three general choices: Building, Quarter, and Sett.
/// Filters can also restrict what kind of area can be chosen if Building or
/// Quarter is selected (e.g. a Building or Quarter of a particular QType)
/// TODO: should other filters than QType(s) be possible?
/// TODO: may need to change .json files to specify QType filters
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Area {
    Building(Vec<QType>),
    Quarter(Vec<QType>),
    Sett,
}

impl Area {
    /// Upgrade to a wider-scale Area.
    fn upgrade(self) -> Area {
        match self {
            Area::Building(v) => Area::Quarter(v),
            _ => Area::Sett,
        }
    }
}

/// A trait for targeting Areas with effects
pub trait Targeted {
    fn kill(&mut self, num: i64);
    fn damage(&mut self, num: i64);
    fn riot(&mut self, num: i64);
    fn grow(&mut self);
    fn build(&mut self);
}

/// An enum representing the rolled result of an effect,
/// which can then be passed to the appropriate area by the manager
/// to be processed on the next step().
/// TODO: since all effects are processed on the step(),
/// TODO: should there be a trait for stepping?
/// TODO: fix documentation
pub enum RolledEffect {
    /// Kill $1 people in $2 area
    Kill(EffectStep, Area),
    /// Damage $1 buildings in $2 area
    Damage(EffectStep, Area),
    /// Slow tickers $1% each turn for $2 turns in $3 area
    Riot(EffectStep, Area),
    /// Boost growth $1% each turn for $2 turns in $3 area
    Grow(EffectStep, Area),
    /// Boost build speed $1% each turn for $2 turns in $3 area
    Build(EffectStep, Area),
    /// Boost gold gain $1% each turn for $2 turns with a one-turn $3 boost
    Gold(EffectStep, EffectStep),
    /// Add hero $1 to building in $3 area
    Hero(i32, String, Area),
    /// Add item worth $1 to building in $3 area
    Item(f64, items::ItemType, i32, Area),
}

impl RolledEffect {
    /// Create a new RolledEffect::Kill from the given arguments.
    fn kill(dead: &str, viralpt: Option<i64>, area: Area) -> RolledEffect {
        let mut ar = area;
        let roll = Roller::new(dead);
        let x : i64 = roll.total();
        if let Some(v) = viralpt {
            // if roll beats viral, "boost" the area up
            if x >= v {
                ar = ar.upgrade()
            }
        }
        // EffectStep takes a %, so divide by 100
        let change = (x as f64 / 100_f64).max(0f64);
        RolledEffect::Kill(EffectStep::new(change, 1), ar)
    }

    /// Create a new RolledEffect::Damage from the given arguments.
    fn damage(crumbled: &str, viralpt: Option<i64>, area: Area) -> RolledEffect {
        let mut ar = area;
        let roll = Roller::new(crumbled);
        let x : i64 = roll.total();
        if let Some(v) = viralpt {
            // if roll beats viral, "boost" the area up
            if x >= v {
                ar = ar.upgrade()
            }
        }
        // EffectStep takes a %, so divide by 100
        let change = (x as f64 / 100_f64).max(0f64);
        RolledEffect::Damage(EffectStep::new(change, 1), ar)
    }

    /// Create a new RolledEffect::Riot from the given arguments.
    fn riot(steps: &str, prod: f64, area: Area) -> RolledEffect {
        let roll = Roller::new(steps);
        let x : i64 = roll.total();
        RolledEffect::Riot(EffectStep::new(prod, x as usize), area)
    }

    /// Create a new RolledEffect::Grow from the given arguments.
    fn grow(bonus: &str, area: Area) -> RolledEffect {
        let roll = Roller::new(bonus);
        let x : i64 = roll.total();
        // divide by 100, add 100% to create boost
        let change = (x as f64 / 100_f64).max(0f64) + 1f64;
        RolledEffect::Grow(EffectStep::new(change, 1), area)
    }

    /// Create a new RolledEffect::Build from the given arguments.
    fn build(bonus: &str, area: Area) -> RolledEffect {
        let roll = Roller::new(bonus);
        let x : i64 = roll.total();
        // divide by 100, add 100% to create boost
        let change = (x as f64 / 100_f64).max(0f64) + 1f64;
        RolledEffect::Build(EffectStep::new(change, 1), area)
    }

    /// Create a new RolledEffect::Gold from the given arguments.
    fn gold(value: &str, bonus: f64, steps: &str) -> RolledEffect {
        let mut roll = Roller::new(steps);
        let stepx : i64 = roll.total();
        roll = Roller::new(value);
        let valuex : i64 = roll.total();
        // first param is % bonus over steps, second param is absolute immediate bonus
        RolledEffect::Gold(EffectStep::new(bonus, stepx as usize), EffectStep::new(valuex as f64, 1))
    }

    /// Create a new RolledEffect::Hero from the given arguments.
    fn hero(level: &str, classes: &[String]) -> RolledEffect {
        let roll = Roller::new(level);
        let x : i64 = roll.total();
        let class = rand::thread_rng().choose(&classes)
            .expect("Hero provided without any possible classes!");
        //TODO: replace with proper, class-based building choice
        let bldgqs = match class.as_str() {
            "Cleric" | "Druid" | "Monk" => vec![QType::Residential, QType::Port],
            "Fighter" | "Assassin" => vec![QType::Port, QType::Administrative],
            "Paladin" | "Ranger" => vec![QType::Residential, QType::Port, QType::Administrative],
            "Mage" | "Illusionist" => vec![QType::Academic],
            "Thief" => vec![QType::Industrial, QType::Port, QType::Administrative],
            "Bard" => vec![QType::Residential, QType::Academic],
            _ => vec![], //FIXME: dangerous!
        };
        RolledEffect::Hero(x as i32, class.clone(), Area::Building(bldgqs))
    }

    /// Create a new RolledEffect::Item from the given arguments.
    fn item(value: &str, kind: &[String], magical: f64) -> RolledEffect {
        let roll = Roller::new(value);
        let x : i64 = roll.total();
        let mut pow = 0i32;
        if magical < 1.0f64 {
            // take the inverse of magical and compute a bool with a 1 in 1/magical chance
            // this is the same as checking if a random number between 1 and 100 is less
            // than magical.
            while rand::thread_rng().gen_weighted_bool(magical.recip() as u32) && pow < 6 {
                // keep increasing the power level as long as the rolls succeed
                pow += 1;
            }
        } else {
            // if the item is guaranteed magical, set it to the maximum level
            pow = 6;
        }
        let kind = rand::thread_rng().choose(&kind)
            .expect("Item provided without any possible kinds!");
        let itemtype : items::ItemType = kind.parse()
            .expect("Kind of item not a valid choice!");
        let bldgqs = match itemtype {
            items::ItemType::Book | items::ItemType::HolyRelic => vec![QType::Academic],
            items::ItemType::Art => vec![QType::Residential, QType::Academic],
            items::ItemType::Magic => QType::get_qtypes(true),  // all qtypes as vec
            //weapons and armour
            _ => vec![QType::Industrial, QType::Port, QType::Administrative],
        };
        RolledEffect::Item(x as f64, itemtype, pow, Area::Building(bldgqs))
    }

}

/// A struct implementing Iterator to return effect steps.
/// Can be combined or chained with other EffectStep structs
/// to produce a varied series of boosts.
///
/// ```
/// use podesta::effects::EffectStep;
///
/// let e = EffectStep::new(1.5, 4);
/// assert!(e.next(), Some(1.5));
/// assert!(e.next(), Some(1.5));
/// let f = EffectStep::new(2, 1);
/// e.combine(f);
/// assert!(e.next(), Some(3));
/// assert!(e.next(), Some(1.5));
/// assert!(e.next(), None);
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EffectStep {
    steps: Vec<f64>,
}

enum CombineOp {
    Add,
    Mul,
}

impl EffectStep {
    pub fn new(boost: f64, nsteps: usize) -> EffectStep {
        EffectStep {
            steps: vec![boost; nsteps],
        }
    }

    /// Take two overlapping EffectSteps and perform the op on each step in other with self.
    /// The new EffectStep is the length of the longer of self and other: additional
    /// elements (past the length of the shorter EffectStep) are appended as-is.
    fn combine(self, other: EffectStep, op: CombineOp) -> EffectStep {
        let v = if self.steps.len() > other.steps.len() {
            let mut v_part = self.steps.iter().zip(other.steps.iter())
                .map(|(x, y)| match op {
                    CombineOp::Add => *x + *y,
                    CombineOp::Mul => *x * *y,
                }).collect::<Vec<_>>();
            v_part.extend_from_slice(&self.steps[other.steps.len()..]);
            v_part
        } else if other.steps.len() > self.steps.len() {
            let mut v_part = other.steps.iter().zip(self.steps.iter())
                .map(|(x, y)| match op {
                    CombineOp::Add => *x + *y,
                    CombineOp::Mul => *x * *y,
                }).collect::<Vec<_>>();
            v_part.extend_from_slice(&other.steps[self.steps.len()..]);
            v_part
        } else {
            self.steps.iter().zip(other.steps.iter())
                .map(|(x, y)| match op {
                    CombineOp::Add => *x + *y,
                    CombineOp::Mul => *x * *y,
                }).collect::<Vec<_>>()
        };
        EffectStep {
            steps: v,
        }
    }
}

impl Add for EffectStep {
    type Output = EffectStep;
    fn add(self, other: EffectStep) -> EffectStep {
        self.combine(other, CombineOp::Add)
    }
}

impl AddAssign for EffectStep {
    fn add_assign(&mut self, other: EffectStep) {
        *self = (self.clone()).combine(other, CombineOp::Add);
    }
}

impl MulAssign for EffectStep {
    fn mul_assign(&mut self, other: EffectStep) {
        *self = (self.clone()).combine(other, CombineOp::Mul);
    }
}

impl Mul for EffectStep {
    type Output = EffectStep;
    fn mul(self, other: EffectStep) -> EffectStep {
        self.combine(other, CombineOp::Mul)
    }
}

impl Iterator for EffectStep {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        if self.steps.len() == 0 {
            None
        } else {
            Some(self.steps.remove(0))
        }
    }
}

/// Struct for tracking duration and intensity of non-instant effects.
/// These include Riot, Grow, Build and Gold.
/// Struct is created by an event and then passed to the target.
#[derive(Serialize, Deserialize, Debug)]
pub struct EffectFlags {
    //TODO: consider changing everything to iterators
    pub grow: EffectStep,
    pub build: EffectStep,
    pub gold: EffectStep,
    pub grow_bonus: EffectStep,
    pub build_bonus: EffectStep,
    pub gold_bonus: EffectStep,
}

impl EffectFlags {
    pub fn new(gw: EffectStep, bu: EffectStep, gd: EffectStep,
               pb: EffectStep, bb: EffectStep, gb: EffectStep) -> EffectFlags
    {
        EffectFlags {
            grow: gw,
            build: bu,
            gold: gd,
            grow_bonus: pb,
            build_bonus: bb,
            gold_bonus: gb,
        }
    }
}

impl default::Default for EffectFlags {
    fn default() -> EffectFlags {
        EffectFlags::new(EffectStep::new(1.0, 1),
                         EffectStep::new(1.0, 1),
                         EffectStep::new(1.0, 1),
                         EffectStep::new(0.0, 1),
                         EffectStep::new(0.0, 1),
                         EffectStep::new(0.0, 1))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventEffect {
    Kill { dead: String, viralpt: Option<i64>, area: Area },
    Damage { crumbled: String, viralpt: Option<i64>, area: Area },
    Riot { steps: String, prod: f64, area: Area },
    Grow { bonus: String, area: Area },
    Build { bonus: String, area: Area },
    Gold { value: String, bonus: f64, steps: String },
    Hero { level: String, classes: Vec<String> },
    Item { value: String, kind: Vec<String>, magical: f64 },
}

impl EventEffect {
    pub fn activate(&self) -> RolledEffect {
        match *self {
            EventEffect::Kill { ref dead, viralpt, ref area } =>
                RolledEffect::kill(dead, viralpt, area.clone()),
            EventEffect::Damage { ref crumbled, viralpt, ref area } =>
                RolledEffect::damage(crumbled, viralpt, area.clone()),
            EventEffect::Riot { ref steps, prod, ref area } =>
                RolledEffect::riot(steps, prod, area.clone()),
            EventEffect::Grow { ref bonus, ref area } =>
                RolledEffect::grow(bonus, area.clone()),
            EventEffect::Build { ref bonus, ref area } =>
                RolledEffect::build(bonus, area.clone()),
            EventEffect::Gold { ref value, bonus, ref steps } =>
                RolledEffect::gold(value, bonus, steps),
            EventEffect::Hero { ref level, ref classes } =>
                RolledEffect::hero(level, classes),
            EventEffect::Item { ref value, ref kind, magical } =>
                RolledEffect::item(value, kind, magical),
        }
    }
}
