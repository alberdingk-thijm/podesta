use rand;
use rand::Rng;
use rouler::Roller;
use people;
//use buildings;
use quarters;
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
    Building(Vec<quarters::QType>),
    Quarter(Vec<quarters::QType>),
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
    Item(f64, Area),
}

impl RolledEffect {
    /// Create a new RolledEffect::Kill from the given arguments.
    fn kill(dead: &str, viralpt: Option<i64>, area: Area) -> RolledEffect {
        let mut ar = area;
        let mut roll = Roller::new(dead);
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
        let mut roll = Roller::new(crumbled);
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
        let mut roll = Roller::new(steps);
        let x : i64 = roll.total();
        RolledEffect::Riot(EffectStep::new(prod, x as usize), area)
    }

    /// Create a new RolledEffect::Grow from the given arguments.
    fn grow(bonus: &str, area: Area) -> RolledEffect {
        let mut roll = Roller::new(bonus);
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
        let class = rand::thread_rng().choose(&classes);
        //TODO: replace with proper, class-based building choice
        let bldgqs = match class.unwrap().as_str() {
            "Cleric" | "Druid" | "Monk" => vec![quarters::QType::Residential, quarters::QType::Port],
            "Fighter" | "Assassin" => vec![quarters::QType::Port, quarters::QType::Administrative],
            "Paladin" | "Ranger" => vec![quarters::QType::Residential, quarters::QType::Port, quarters::QType::Administrative],
            "Mage" | "Illusionist" => vec![quarters::QType::Academic],
            "Thief" => vec![quarters::QType::Industrial, quarters::QType::Port, quarters::QType::Administrative],
            "Bard" => vec![quarters::QType::Residential, quarters::QType::Academic],
            _ => vec![],
        };
        RolledEffect::Hero(x as i32, class.unwrap().clone(), Area::Building(bldgqs))
    }

    /// Create a new RolledEffect::Item from the given arguments.
    fn item(value: &str, kind: &[String], magical: f64) -> RolledEffect {
        //TODO
        RolledEffect::Item(0.0, Area::Building(vec![]))
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

pub struct Effect<T: Targeted> {
    pub target: T,
    pub etype: EventEffect,
}

#[allow(unused_variables)]
impl EventEffect {
    pub fn activate(&self) -> RolledEffect {
        match *self {
            //TODO: replace placeholder values with proper code
            EventEffect::Kill { ref dead, viralpt, ref area } => {
                //TODO:
                //roll dead and store as f64
                //if > Some(viralpt), make area larger
                //else, use given area
                RolledEffect::kill(dead, viralpt, area.clone())
            },
            EventEffect::Damage { ref crumbled, viralpt, ref area } => {
                RolledEffect::damage(crumbled, viralpt, area.clone())
            },
            EventEffect::Riot { ref steps, prod, ref area } => {
                RolledEffect::riot(steps, prod, area.clone())
            },
            EventEffect::Grow { ref bonus, ref area } => {
                RolledEffect::grow(bonus, area.clone())
            },
            EventEffect::Build { ref bonus, ref area } => {
                RolledEffect::build(bonus, area.clone())
            },
            EventEffect::Gold { ref value, bonus, ref steps } =>
                RolledEffect::gold(value, bonus, steps),
            EventEffect::Hero { ref level, ref classes } =>
                RolledEffect::hero(level, classes),
            EventEffect::Item { ref value, ref kind, magical } =>
                RolledEffect::item(value, kind, magical),
        }
    }
}

#[allow(dead_code, unused_variables)]
impl<T: Targeted> Effect<T> {
    pub fn new(tgt: T, etype: EventEffect) -> Effect<T> {
        Effect { target: tgt, etype: etype }
    }

    fn kill(&mut self, dead: &str, viralpt: Option<i64>) {
        // get the roll
        let mut roll = Roller::new(dead);
        let mut x : i64 = roll.total();
        if let Some(v) = viralpt {
            if x >= v {
                x += roll.reroll();
            }
        }
        // perform it on the target
        self.target.kill(x)
    }

    fn event_damage(&mut self, crumbled: &str, viralpt: Option<i64>) {
        // get the roll
        let mut roll = Roller::new(crumbled);
        let mut x: i64 = roll.total();
        if let Some(v) = viralpt {
            if x >= v {
                x += roll.reroll();
            }
        }
        // perform it on the area
        self.target.damage(x);
    }

    fn event_riot(&mut self, steps: &str, prod: f64) {
        // get the roll
        let roll = Roller::new(steps);
        unimplemented!()
    }

    fn event_grow(&mut self, bonus: &str) {
        unimplemented!()
    }

    fn event_build(&mut self, bonus: &str) {
        unimplemented!()
    }

    fn event_gold(&mut self, value: &str, bonus: f64, steps: &str) {
        // get the rolls
        let valroll = Roller::new(value);
        let steproll = Roller::new(steps);
        // sett.gold += valroll
        // next steproll steps, gold earned * bonus
        unimplemented!()
    }

    fn event_hero(&mut self, level: &str, classes: &Vec<people::Class>) {
        // get the roll
        let lvlroll = Roller::new(level);
        // choose the class
        let r = rand::thread_rng().gen_range(0, classes.len());
        // let h = people::Hero::new(class: classes[r]);
        // add new hero to building
        unimplemented!()
    }

    fn event_item(&mut self, value: &str, magical: f64) {
        let roll = Roller::new(value);
        unimplemented!()
    }
}
