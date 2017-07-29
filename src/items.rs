use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
/// A unique item in the settlement.
pub struct Item {
    pub name: String,
    pub age: i32,
    pub kind: ItemType,
    pub power: i32,
    pub worth: f64,
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
               if self.power > 0 { format!("+{} ", self.power) } else { "" },
               self.kind, self.worth)
    }
}
