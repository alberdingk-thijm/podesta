use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
/// A unique item in the settlement.
pub struct Item {
    pub name: String,
    pub age: i32,
    pub kind: ItemType,
    /// The magical power of the item: 0 if non-magical, up to +6 otherwise.
    pub power: i32,
    pub worth: f64,
}

impl Item {
    pub fn new(n: &str, k: ItemType, p: i32, w: f64) -> Item {
        Item {
            name: n.to_string(),
            age: 0,
            kind: k,
            power: p,
            worth: w,
        }
    }

    /// Execute a timestep for the item, returning the value in gold
    /// it generates that step.
    pub fn step(&mut self) {
        self.age += 1;
        //TODO: placeholder increment
    }

    /// Collect gold. Return a % of the item's worth based on its power.
    pub fn collect_gold(&self) -> f64 {
        self.worth * (0.01 * self.power as f64)
    }
}

macro_attr! {
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq,
         IterVariants!(ItemTypeVariants),
         IterVariantNames!(ItemTypeVariantNames),
         EnumDisplay!, EnumFromStr!)]
    /// An enum representing common types of Item.
    pub enum ItemType {
        Book,
        Art,
        HolyRelic,
        Magic,
        LightArmour,
        HeavyArmour,
        LightWeapon,
        HeavyWeapon,
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, a {}{} (worth {} gold)",
               self.name,
               if self.power > 0 { format!("+{} ", self.power) } else { String::from("") },
               self.kind, self.worth)
    }
}
