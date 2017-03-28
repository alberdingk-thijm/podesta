use rand;
use rand::Rng;
use rouler::Roller;
use people;
use buildings;
use std::str;
use serde::de;

#[derive(Serialize, Deserialize, Debug)]
pub enum Area {
    Hero,
    Building,
    Quarter,
    Sett,
}

impl Area {
    /// Return the struct we want to mutate based on the area and
    /// the calling structure.
    ///
    /// # Examples
    ///
    /// ```
    /// use podesta::effects;
    /// use podesta::buildings;
    ///
    /// let mut b = buildings::Building {
    ///     name: 'foo',
    ///     id: 1,
    ///     btype: quarters::Residential,
    ///     preq: None,
    ///     cost: 100.0,
    ///     build: 1.0,
    ///     events: vec!()
    /// };
    /// let a = effects::Area::Building;
    /// assert_eq!(a.target(b).name , 'foo')
    /// ```
    pub fn target<T>(&self, caller: &mut buildings::Building) -> &mut T
    where T: HasTarget
    {
        unimplemented!()
    }
}

/// A trait for targeting areas with effects
pub trait HasTarget {

    fn kill(&mut self, num: i32);

    fn damage(&mut self, num: i32);

    fn riot(&mut self, num: i32);

    fn grow(&mut self);

    fn build(&mut self);
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventEffect {
    Kill { dead: String, viralpt: Option<i32>, area: Area },
    Damage { crumbled: String, viralpt: Option<i32>, area: Area },
    Riot { steps: String, prod: f64, area: Area },
    Grow { bonus: String, area: Area },
    Build { bonus: String, area: Area },
    Gold { value: String, bonus: f64, steps: String },
    Hero { level: String, classes: Vec<people::Class> },
    Item { value: String, magical: f64 },
}


/*
impl<'a> de::Deserialize for Roller<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Roller<'a>, D::Error>
    where D: de::Deserializer
    {
        // Return a new roller based on the deserialized string
        let r = try!(Roller::new(de::Deserialize::deserialize(deserializer)));
        Ok(r)
    }
}
*/
impl EventEffect {
    pub fn activate(&self, caller: &mut buildings::Building) {
        match *self {
            EventEffect::Kill { ref dead, viralpt, ref area } => {
                let ref mut tgt = area.target(caller);
                event_kill(tgt, dead, viralpt)
            },
            EventEffect::Damage { ref crumbled, viralpt, ref area } => {
                let ref mut tgt = area.target(caller);
                event_damage(tgt, crumbled, viralpt)
            },
            EventEffect::Riot { ref steps, prod, ref area } => {
                let ref mut tgt = area.target(caller);
                event_riot(tgt, steps, prod)
            },
            EventEffect::Grow { ref bonus, ref area } => {
                let ref mut tgt = area.target(caller);
                event_grow(tgt, bonus)
            },
            EventEffect::Build { ref bonus, ref area } => {
                let ref mut tgt = area.target(caller);
                event_build(tgt, bonus)
            },
            EventEffect::Gold { ref value, bonus, ref steps } =>
                event_gold(value, bonus, steps),
            EventEffect::Hero { ref level, ref classes } =>
                event_hero(level, classes),
            EventEffect::Item { ref value, magical } =>
                event_item(value, magical),
        }
    }
}

fn event_kill<T: HasTarget>(tgt: &mut T, dead: &str, viralpt: Option<i32>) {
    // get the roll
    let mut roll = Roller::new(dead);
    let mut x : i32 = roll.total();
    if x >= viralpt {
        x += roll.reroll().total();
    }
    // perform it on the target
    // tgt.kill(x)
    unimplemented!()
}

fn event_damage<T: HasTarget>(tgt: &mut T, crumbled: &str, viralpt: Option<i32>) {
    // get the roll
    let mut roll = Roller::new(crumbled);
    let mut x : i32 = roll.total();
    if x >= viralpt {
        x += roll.reroll().total();
    }
    // perform it on the area
    // tgt.damage(x);
    unimplemented!()
}

fn event_riot<T: HasTarget>(tgt: &mut T, steps: &str, prod: f64) {
    // get the roll
    let mut roll = Roller::new(steps);
    unimplemented!()
}

fn event_grow<T: HasTarget>(tgt: &mut T, bonus: &str) {
    unimplemented!()
}

fn event_build<T: HasTarget>(tgt: &mut T, bonus: &str) {
    unimplemented!()
}

fn event_gold(value: &str, bonus: f64, steps: &str) {
    // get the rolls
    let valroll = Roller::new(value);
    let steproll = Roller::new(steps);
    // sett.gold += valroll
    // next steproll steps, gold earned * bonus
    unimplemented!()
}

fn event_hero(level: &str, classes: &Vec<people::Class>) {
    // get the roll
    let lvlroll = Roller::new(level);
    // choose the class
    let r = rand::thread_rng().gen_range(0, classes.len());
    // let h = people::Hero::new(class: classes[r]);
    // add new hero to building
    unimplemented!()
}

fn event_item(value: &str, magical: f64) {
    let roll = Roller::new(value);
    unimplemented!()
}

impl str::FromStr for EventEffect {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // first term is eventeffect name,
        // second is which value to use
        unimplemented!()
    }
}
