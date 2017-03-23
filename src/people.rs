
#[derive(Debug)]
pub struct Hero {
    pub name: String,
    pub age: String,
    pub race: Race,
    pub class: Class,
    pub activity: Activity,
}

#[derive(Debug)]
pub enum Race {
    Dwarf,
    Elf,
    Gnome,
    Halfelf,
    Halfling,
    Halforc,
    Human,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Activity {
    Working,
    Governing,
    Trading,
    Adventuring,
    Resting,
    Dead,
}
