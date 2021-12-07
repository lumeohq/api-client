use std::str::FromStr;

use serde::{
    de::{Deserialize, Deserializer, Error},
    ser::{Serialize, Serializer},
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl FromStr for Resolution {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split('x').collect::<Vec<_>>()[..] {
            [width, height] => match (width.parse(), height.parse()) {
                (Ok(width), Ok(height)) => Ok(Resolution { width, height }),
                _ => Err("Resolution must be a number".into()),
            },
            _ => Err(format!("Bad resolution format: {}", s)),
        }
    }
}

impl Serialize for Resolution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}x{}", self.width, self.height);

        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Resolution {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Resolution::from_str(&s).map_err(Error::custom)
    }
}
