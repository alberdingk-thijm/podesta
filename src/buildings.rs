use std::fmt;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Building {
    pub name: String,
    pub id: i32,
    #[serde(rename = "type")]
    pub btype: BType,  // use serde macro to match type
    pub preq: Option<Vec<String>>,  // can be null
    pub cost: f32,
    pub build: f32,
    pub events: Vec<EventChance>,
    //#[serde(with = "hexflags")]
    //pub flags: Vec<BFlags>,  // should be hex
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BType {
    Residential,
    Industrial,
    Port,
    Academic,
    Administrative,
}

impl fmt::Display for BType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            BType::Residential => "RESIDENTIAL",
            BType::Industrial => "INDUSTRIAL",
            BType::Port => "PORT",
            BType::Academic => "ACADEMIC",
            BType::Administrative => "ADMINISTRATIVE",
        })
    }
}

impl str::FromStr for BType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RESIDENTIAL" => Ok(BType::Residential),
            "INDUSTRIAL" => Ok(BType::Industrial),
            "PORT" => Ok(BType::Port),
            "ACADEMIC" => Ok(BType::Academic),
            "ADMINISTRATIVE" => Ok(BType::Administrative),
            _ => Err(()),
        }
    }
}

/*
impl ::serde::Serialize for BType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where S: ::serde::Serializer {
        // serialize enum as string in all caps
        let name = self.to_string();
        serializer.serialize_str(&name)
    }
}

impl ::serde::Deserialize for BType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where D: ::serde::Deserializer {

        struct Visitor;

        impl ::serde::de::Visitor for Visitor {
            type Value = BType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string")
            }

            fn visit_str<E>(self, value: &str) -> Result<BType, E>
                where E: ::serde::de::Error
            {
                match value.parse::<BType>() {
                    Ok(t) => Ok(t),
                    Err(_) => Err(E::custom(
                            format!("No BType for {}", value))),
                }
            }
        }

        // Deserialize the enum from a str.
        deserializer.deserialize_str(Visitor)
    }
}
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct EventChance {
    name: String,
    chance: f32,
}
/*
pub mod hexflags {
    use serde::{self, Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(hex: i32, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let s = format!("{:x}", hex);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<D>(deserializer: D) -> Result<i32, D::Error>
        where D: Deserializer
    {
        let s = String::deserialize(deserializer)?;
    }
}*/

impl Building {

}
