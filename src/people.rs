
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Hero {
    pub name: String,
    pub age: i32,
    pub race: Race,
    pub class: Class,
    pub activity: Activity,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Race {
    Dwarf,
    Elf,
    Gnome,
    Halfelf,
    Halfling,
    Halforc,
    Human,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Class {
    Cleric,
    Druid,
    Fighter,
    Assassin,
    Paladin,
    Ranger,
    Mage,
    Illusionist,
    Thief,
    Monk,
    Bard,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Activity {
    Working,
    Governing,
    Trading,
    Adventuring,
    Resting,
    Dead,
}

impl Hero {
    #[allow(dead_code)]
    pub fn execute_timestep(&self) {
        match self.activity {
            Activity::Working => (),
            Activity::Governing => (),
            Activity::Trading => (),
            Activity::Adventuring => (),
            Activity::Resting => (),
            Activity::Dead => (),
        }
    }
}
