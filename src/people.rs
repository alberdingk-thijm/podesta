use std::fmt;
use std::default;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
/// A hero of the settlement.
pub struct Hero {
    pub name: String,
    /// Once age reaches Race::max_age, the hero dies.
    pub age: i32,
    pub level: i32,
    pub race: Race,
    pub class: Rc<Class>,
    /// What the hero is currently doing.
    pub activity: Activity,
}

macro_attr! {
    #[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq,
             IterVariants!(RaceVariants),
             IterVariantNames!(RaceVariantNames),
             EnumDisplay!, EnumFromStr!)]
    /// The race of a person.
    /// Different races get different bonuses to their activities.
    pub enum Race {
        /// Dwarves are hardy adventurers and strong builders.
        Dwarf,
        /// Elves are keen adventurers and good governors.
        Elf,
        /// Gnomes are hardy adventurers and fine traders.
        Gnome,
        /// Half elves are fertile and good governors.
        Halfelf,
        /// Halflings are fine traders and fertile.
        Halfling,
        /// Half orcs are strong builders and fertile.
        Halforc,
        /// Humans are fertile and hardy adventurers.
        Human,
    }
}

impl Race {
    /// Return the maximum natural age of a person based on their race.
    fn max_age(&self) -> i32 {
        match self {
            // Implemented for the case where someone might prefer
            // that different races can be older than one another.
            // TODO: make a game setting
            _ => 10
        }
    }
}
/*
#[derive(Serialize, Deserialize, Debug)]
/// The class of a person.
/// Different classes provide different bonuses.
/// Class bonuses are TODO.
pub enum Class {
    /// Clerics help fight death and sickness and make good governors.
    Cleric,
    /// Druids help fight death and sickness and travel for longer.
    Druid,
    /// Fighters make good workers and adventurers.
    Fighter,
    /// Assassins kill people and make good adventurers.
    Assassin,
    /// Paladins are immune to illness and make good governors.
    Paladin,
    /// Rangers make strong adventurers and can travel for longer.
    Ranger,
    /// Mages help produce magical items and make good governors.
    Mage,
    /// Illusionists help produce magical items and make good adventurers.
    Illusionist,
    /// Thieves steal gold and make keen adventurers.
    Thief,
    /// Monks make good workers and can travel for longer, but cannot govern.
    Monk,
    /// Bards help produce magical items and make good workers.
    Bard,
    /// Merchants are able to trade.
    Merchant,
}
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub id: i32,
    pub desc: String,
    pub races: Vec<String>,
    #[serde(rename = "buildings")]
    pub bldgs: Vec<String>,
    pub items: Vec<String>,
    pub age: i32,
    #[serde(rename = "activities")]
    pub act_boosts: HashMap<String, f64>,
    pub powers: Vec<String>,
}

impl Class {
    fn new(n: &str, id: i32, d: &str,
           races: Vec<String>, bldgs: Vec<String>, items: Vec<String>,
           age: i32, ab: HashMap<String, f64>, powers: Vec<String>) -> Class {
        Class {
            name: n.to_string(),
            id: id,
            desc: d.to_string(),
            races: races,
            bldgs: bldgs,
            items: items,
            age: age,
            act_boosts: ab,
            powers: powers,
        }
    }
}

impl default::Default for Class {
    fn default() -> Class {
        let mut acts = HashMap::new();
        acts.insert(String::from("Working"), 1f64);
        acts.insert(String::from("Governing"), 1f64);
        acts.insert(String::from("Trading"), 0f64);
        acts.insert(String::from("Adventuring"), 1f64);
        acts.insert(String::from("Resting"), 1f64);
        acts.insert(String::from("Treasure"), 1f64);
        Class::new("DEFAULT", 1, "defaultdesc",
                   vec!["ALL".to_string()], vec!["ALL".to_string()], vec!["ALL".to_string()],
                   0, acts, vec![])
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Activity {
    Working,
    Governing,
    /// Trading lasts for a set number of turns.
    Trading(i32),
    /// Adventuring lasts for a set number of turns.
    Adventuring(i32),
    /// Resting tracks the gradually shrinking chance of death.
    Resting(i32),
    /// Treasure creates a special item or gold.
    Treasure(i32),
    /// Dying reports the cause of death.
    Dying(String),
    Dead,
}

impl Activity {

    /// Return a String describing the cause of death based on the
    /// Activity last performed.
    ///
    /// # Examples
    ///
    /// ```
    /// use podesta::people::Activity;
    /// let a = Activity::Working;
    /// assert_eq!(a.autopsy(), "overworked".to_string())
    /// ```
    pub fn autopsy(&self) -> String {
        match *self {
            Activity::Working => "overworked",
            Activity::Governing => "under suspicious circumstances",
            Activity::Trading(_) => "on a trade expedition",
            Activity::Adventuring(_) => "in a deadly dungeon",
            Activity::Resting(_) => "illness",
            Activity::Treasure(_) => "greed",
            _ => "old age",
        }.to_string()
    }
}

impl fmt::Display for Hero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, a {}{} level {:?} {:?} - currently {:?}",
               self.name, self.level,
               // get level suffix
               match { self.level % 10 } {
                   1 if self.level % 100 != 11 => "st",
                   2 if self.level % 100 != 12 => "nd",
                   3 if self.level % 100 != 13 => "rd",
                   _ => "th"
               },
               self.race,
               self.class,
               self.activity)
    }
}

impl Hero {
    // Multiplier against level for determining time away
    // trading or adventuring
    // const AWAYMOD: i32 = 4;
    fn awaymod() -> i32 { 4 }
    // Multiplier against age for determining chance of resting or dying
    // Heroes die naturally at AGEMOD * Race::max_age() steps.
    // const AGEMOD: i32 = 25;
    fn agemod() -> i32 { 25 }

    pub fn new(n: &str, lvl: i32, race: Race, class: Rc<Class>) -> Hero {
        Hero {
            name: n.to_string(),
            age: class.age * Hero::agemod(),
            level: lvl,
            race: race,
            class: class,
            activity: Activity::Working,
        }
    }

    /// Execute a timestep, aging the hero and changing their activity based on
    /// a provided percentile roll r (between 1 and 100, inclusive).
    /// TODO: use new class format
    /// ```
    /// use podesta::people;
    /// let h = people::Hero::new {
    ///     "George",
    ///     1,
    ///     people::Race::Human,
    ///     people::Class::default()
    /// };
    /// assert_eq!(h.activity, people::Activity::Working);
    /// let mut r = 50;
    /// h.step(r);
    /// assert_eq!(h.activity, people::Activity::Adventuring);
    /// ```
    pub fn step(&mut self, r: i32) {
        if self.age >= Hero::agemod() * self.race.max_age() {
            self.activity = Activity::Dying("old age".to_string());
            // TODO: Log the death
            // Don't need to do the rest so just return
            return;
        }
        let next = match self.activity {
            Activity::Working => {
                if r < 2 {
                    Activity::Dying(self.activity.autopsy())
                } else if r < (self.age / Hero::agemod() + 4) {
                    // TODO: replace hard-coded numbers
                    Activity::Resting(10)
                } else if r < (self.age / Hero::agemod() + 29) {
                    let away = Hero::awaymod() * self.level;
                    //TODO: allow for possible hybrids
                    if self.class.powers.contains(&"Trading".to_string()) {
                        Activity::Trading(away)
                    } else {
                        Activity::Adventuring(away)
                    }
                } else {
                    Activity::Working
                }
                // Manager:
                // Growth and build speed increase
            },
            Activity::Governing => {
                if r < 3 {
                    Activity::Dying(self.activity.autopsy())
                } else if r < (self.age / Hero::agemod() + 5) {
                    Activity::Resting(15) }
                else {
                    Activity::Governing
                }
                // Manager:
                // Growth and build speed increase
            },
            Activity::Trading(steps) => {
                if steps > 0 {
                    if r < 3 {
                        Activity::Dying(self.activity.autopsy())
                    } else if r < (self.age / Hero::agemod() + 5) {
                        Activity::Resting(15)
                    } else {
                        Activity::Trading(steps - 1)
                    }
                } else {
                    Activity::Working
                }
                // Manager:
                // Growth, build and gold increase
                // Immune to town effects
            },
            Activity::Adventuring(steps) => {
                if steps > 0 {
                    if r < 6 {
                        Activity::Dying(self.activity.autopsy())
                    } else if r < (self.age / Hero::agemod() + 8) {
                        Activity::Resting(20)
                    } else {
                        Activity::Adventuring(steps - 1)
                    }
                } else {
                    self.level += 1;
                    Activity::Treasure(self.level)
                }
                // Manager:
                // Immune to town effects
            },
            Activity::Resting(steps) => {
                if steps == 0 || r > 74 {
                    Activity::Working
                } else if r < steps {
                    Activity::Dying(self.activity.autopsy())
                } else {
                    Activity::Resting(steps - 1)
                }
            },
            Activity::Treasure(_) => Activity::Working,
            _ => Activity::Dead,
        };
        self.activity = next;
    }

    #[allow(dead_code)]
    /// Promote a hero to the Governing activity.
    ///
    /// # Examples
    ///
    /// ```
    /// use podesta::people;
    /// let h = people::Hero::new(
    ///     "George",
    ///     1,
    ///     people::Race::Human,
    ///     people::Class::Fighter
    /// );
    /// assert_eq!(h.activity, people::Activity::Working);
    /// h.make_governor();
    /// assert_eq!(h.activity, people::Activity::Governing);
    /// ```
    fn make_governor(&mut self) {
        self.activity = Activity::Governing;
    }
}
