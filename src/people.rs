use rand;
use rand::Rng;

#[allow(dead_code)]
#[derive(Debug)]
/// A hero of the settlement.
pub struct Hero {
    pub name: String,
    /// Once age reaches Race::max_age, the hero dies.
    pub age: i32,
    pub level: i32,
    pub race: Race,
    pub class: Class,
    /// What the hero is currently doing.
    pub activity: Activity,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
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

#[allow(dead_code)]
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
    /// Monks make good workers and can travel for longer.
    Monk,
    /// Bards help produce magical items and make good workers.
    Bard,
    /// Merchants are able to trade.
    Merchant,
}

#[allow(dead_code)]
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
    /// Dead reports the cause of death.
    Dead(String),
}

impl Activity {

    /// Return a String describing the cause of death based on the
    /// Activity last performed.
    pub fn autopsy(&self) -> String {
        match *self {
            Activity::Working => "overworked",
            Activity::Governing => "suspicious circumstances",
            Activity::Trading(_) => "on a trade expedition",
            Activity::Adventuring(_) => "in a deadly dungeon",
            Activity::Resting(_) => "succumbed to illness",
            Activity::Dead(_) => "old age",
        }.to_string()
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

    #[allow(dead_code)]
    pub fn execute_timestep(&mut self) {
        if self.age >= Hero::agemod() * self.race.max_age() {
            self.activity = Activity::Dead("old age".to_string());
            // TODO: Log the death
            // Don't need to do the rest so just return
            return;
        }
        // The random number r decides how the Activity should change
        let r = rand::thread_rng().gen_range(1, 101);
        match self.activity {
            Activity::Working => {
                self.activity = if r < 2 {
                    Activity::Dead(self.activity.autopsy()) }
                    else if r < (self.age / Hero::agemod() + 4) {
                        Activity::Resting(10) }
                    // TODO: replace hard-coded numbers
                    else if r < (self.age / Hero::agemod() + 29) {
                        let away = Hero::awaymod() * self.level;
                        match self.class {
                            Class::Merchant => Activity::Trading(away),
                            _ => Activity::Adventuring(away),
                        }
                    } else { Activity::Working };
                // Manager:
                // Growth and build speed increase
            },
            Activity::Governing => {
                self.activity = if r < 3 {
                    Activity::Dead(self.activity.autopsy()) }
                    else if r < (self.age / Hero::agemod() + 5) {
                        Activity::Resting(15) }
                    else { Activity::Governing };
                // Manager:
                // Growth and build speed increase
            },
            Activity::Trading(steps) => {
                self.activity = if steps > 0 {
                    if r < 3 { Activity::Dead(self.activity.autopsy()) }
                    else if r < (self.age / Hero::agemod() + 5) {
                        Activity::Resting(15) }
                    else { Activity::Trading(steps - 1) }
                } else { Activity::Working };
                // Manager:
                // Growth, build and gold increase
                // Immune to town effects
            },
            Activity::Adventuring(steps) => {
                self.activity = if steps > 0 {
                    if r < 6 { Activity::Dead(self.activity.autopsy()) }
                    else if r < (self.age / Hero::agemod() + 8) {
                        Activity::Resting(20)
                    } else { Activity::Adventuring(steps - 1) }
                } else {
                    self.level += 1;
                    self.treasure();
                    Activity::Working
                };
                // Manager:
                // Immune to town effects
            },
            Activity::Resting(steps) => {
                self.activity = if steps == 0 || r > 74 {
                    Activity::Working
                } else if r < steps {
                    Activity::Dead(self.activity.autopsy())
                } else { Activity::Resting(steps - 1) };
            },
            Activity::Dead(_) => {
                // Remain dead
                // Will get cleaned up by manager on next pass
            },
        }
    }

    #[allow(dead_code)]
    /// Promote a hero to the Governing activity.
    fn make_governor(&self) {
        unimplemented!()
    }

    /// Acquire gold and/or an item from adventuring.
    fn treasure(&self) {
        unimplemented!()
    }

    // TODO:
    // Report cause of death
}
