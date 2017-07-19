use rand;
use rand::Rng;
use rouler::Roller;
use people;
use buildings;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub enum Area {
    Building,
    Quarter,
    Sett,
}

impl Area {
    /// Return the struct we want to mutate based on the area and
    /// the calling structure.
    ///
    /// # Examples
    /// TODO: update to new syntax
    /// ```ignore
    /// use podesta::effects;
    /// use podesta::buildings;
    ///
    /// let mut b = buildings::Building {
    ///     name: "foo",
    ///     id: 1,
    ///     btype: quarters::Residential,
    ///     preq: None,
    ///     cost: 100.0,
    ///     bspeed: 1.0,
    ///     events: vec!()
    /// };
    /// let a = effects::Area::Building;
    /// assert_eq!(a.target(b).name , "foo")
    /// ```
    #[allow(unused_variables)]
    pub fn target<T>(&self, caller: &mut buildings::Building) -> &mut T
    where T: Targeted
    {
        unimplemented!()
        /*
        match *self {
            Area::Building => caller,
            Area::Quarter => caller.loc,
            Area::Sett => caller.loc.sett,
        }
        */
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

/*
#[derive(Serialize, Deserialize, Debug)]
pub enum EventEffect {
    KillHeroes { dead: String },
    KillQuarter { dead: String, viralpt: Option<i64> },
    KillSett { dead: String },
    DamageBuilding { crumbled: String, viralpt: Option<i64> },
    DamageQuarter { crumbled: String, viralpt: Option<i64> },
    DamageSett { crumbled: String },
    RiotQuarter { steps: String, prod: f64 },
    RiotSett { steps: String, prod: f64 },
    GrowQuarter { bonus: String },
    GrowSett { bonus: String },
    BuildQuarter { bonus: String },
    BuildSett { bonus: String },
    Gold { value: String, bonus: f64, steps: String },
    Hero { level: String, classes: Vec<people::Class> },
    Item { value: String, magical: f64 },
}
*/

/// Struct for tracking duration and intensity of non-instant effects.
/// These include Riot, Grow, Build and Gold.
/// Struct is created by an event and then passed to the target.
#[derive(Serialize, Deserialize, Debug)]
pub struct EffectFlags {
    pub turns: i32,
    pub grow: f64,
    pub build: f64,
    pub gold: f64,
}

impl EffectFlags {
    fn new(time: i32, gw: f64, bu: f64, gd: f64) -> EffectFlags {
        EffectFlags {
            turns: time,
            grow: gw,
            build: bu,
            gold: gd,
        }
    }

    fn step(&mut self) {
        self.turns -= 1;
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
    pub fn activate(&self) {
        //let e = Effect::new(caller, self);
        match *self {
            EventEffect::Kill { ref dead, viralpt, ref area } => {
                //let ref mut tgt = area.target(caller);
                //event_kill(tgt, dead, viralpt)
            },
            EventEffect::Damage { ref crumbled, viralpt, ref area } => {
                //let ref mut tgt = area.target(caller);
                //event_damage(tgt, crumbled, viralpt)
            },
            EventEffect::Riot { ref steps, prod, ref area } => {
                //let ref mut tgt = area.target(caller);
                //event_riot(tgt, steps, prod)
            },
            EventEffect::Grow { ref bonus, ref area } => {
                //let ref mut tgt = area.target(caller);
                //event_grow(tgt, bonus)
            },
            EventEffect::Build { ref bonus, ref area } => {
                //let ref mut tgt = area.target(caller);
                //event_build(tgt, bonus)
            },
            EventEffect::Gold { ref value, bonus, ref steps } => (),
                //event_gold(value, bonus, steps),
            EventEffect::Hero { ref level, ref classes } => (),
                //event_hero(level, classes),
            EventEffect::Item { ref value, magical } => (),
                //event_item(value, magical),
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
