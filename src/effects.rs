use rand;
use rand::Rng;
use rouler::Roller;
use people;
//use buildings;
use quarters;
use std::str;
use std::default;

/// An enum to determine what part of the settlement the effect should change.
/// There are three general choices: Building, Quarter, and Sett.
/// Filters can also restrict what kind of area can be chosen if Building or
/// Quarter is selected (e.g. a Building or Quarter of a particular QType)
/// TODO: should other filters than QType(s) be possible?
/// TODO: may need to change .json files to specify QType filters
#[derive(Serialize, Deserialize, Debug)]
pub enum Area {
    Building(Vec<quarters::QType>),
    Quarter(Vec<quarters::QType>),
    Sett,
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
    Hero(people::Hero, Area),
    /// Add item worth $1 to building in $3 area
    Item(f64, Area),
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
pub struct EffectStep {
    steps: Vec<f64>,
}

impl EffectStep {
    pub fn new(boost: f64, nsteps: usize) -> EffectStep {
        EffectStep {
            steps: vec![boost; nsteps],
        }
    }

    /// Take two overlapping EffectSteps and multiply each step in other with self.
    /// The new EffectStep is the length of the longer of self and other: additional
    /// elements (past the length of the shorter EffectStep) are appended as-is.
    pub fn combine(&mut self, other: &EffectStep) {
        let v = if self.steps.len() > other.steps.len() {
            let mut v_part = self.steps.iter().zip(other.steps.iter())
                .map(|(x, y)| *x * *y).collect::<Vec<_>>();
            v_part.extend_from_slice(&self.steps[other.steps.len()..]);
            v_part
        } else if other.steps.len() > self.steps.len() {
            let mut v_part = other.steps.iter().zip(self.steps.iter())
                .map(|(x, y)| *x * *y).collect::<Vec<_>>();
            v_part.extend_from_slice(&other.steps[self.steps.len()..]);
            v_part
        } else {
            self.steps.iter().zip(other.steps.iter())
                .map(|(x, y)| *x * *y).collect::<Vec<_>>()
        };
        self.steps = v;
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
    //TODO: flags will naturally run out w/o turns field, and can be easily taken from or chained
    //together
    pub turns: i32,
    pub grow: f64,
    pub build: f64,
    pub gold: f64,
    pub build_bonus: f64,
    pub gold_bonus: f64,
}

impl EffectFlags {
    pub fn new(time: i32, gw: f64, bu: f64, gd: f64, bb: f64, gb: f64) -> EffectFlags {
        EffectFlags {
            turns: time,
            grow: gw,
            build: bu,
            gold: gd,
            build_bonus: bb,
            gold_bonus: gb,
        }
    }

    /// Move a turn forward and reset most values to their defaults.
    pub fn step(&mut self) {
        self.turns -= 1;
        self.build_bonus = 0.0;
        self.gold_bonus = 0.0;
        if self.turns <= 0 {
            // reset to standard levels
            self.grow = 1.0;
            self.build = 1.0;
            self.gold = 1.0;
        }
    }
}

impl default::Default for EffectFlags {
    fn default() -> EffectFlags {
        EffectFlags::new(0, 1.0, 1.0, 1.0, 0.0, 0.0)
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
    Hero { level: String, classes: Vec<people::Class> },
    Item { value: String, magical: f64 },
}

pub struct Effect<T: Targeted> {
    pub target: T,
    pub etype: EventEffect,
}

#[allow(dead_code, unused_variables)]
impl EventEffect {
    pub fn activate(&self) -> RolledEffect {
        //let e = Effect::new(caller, self);
        match *self {
            //TODO: replace placeholder values with proper code
            EventEffect::Kill { ref dead, viralpt, ref area } => {
                RolledEffect::Kill(EffectStep::new(1.0, 1), area)
            },
            EventEffect::Damage { ref crumbled, viralpt, ref area } => {
                RolledEffect::Damage(EffectStep::new(1.0, 1), area)
            },
            EventEffect::Riot { ref steps, prod, ref area } => {
                RolledEffect::Riot(EffectStep::new(prod, 1), area)
            },
            EventEffect::Grow { ref bonus, ref area } => {
                RolledEffect::Grow(EffectStep::new(1.0, 1), area)
            },
            EventEffect::Build { ref bonus, ref area } => {
                RolledEffect::Build(EffectStep::new(1.0, 1), area)
            },
            EventEffect::Gold { ref value, bonus, ref steps } =>
                RolledEffect::Gold(EffectStep::new(1.0, 1), EffectStep::new(bonus, 1)),
            EventEffect::Hero { ref level, ref classes } =>
                RolledEffect::Hero(people::Hero::new("Foo", 1,
                                                     people::Race::Human,
                                                     people::Class::Fighter), Area::Sett),
            EventEffect::Item { ref value, ref magical } =>
                RolledEffect::Item(0.0, Area::Sett),
        }
    }
}

#[allow(dead_code, unused_variables)]
impl<T: Targeted> Effect<T> {
    pub fn new(tgt: T, etype: EventEffect) -> Effect<T> {
        Effect { target: tgt, etype: etype }
    }

    fn event_kill(&mut self, dead: &str, viralpt: Option<i64>) {
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
