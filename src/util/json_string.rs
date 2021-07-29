//! De-/serialization functions to and from json strings.

use serde::{
    de::{Deserialize, DeserializeOwned, Deserializer, Error as _},
    ser::{Error as _, Serialize, Serializer},
};

/// Serialize the given value as a JSON string.
#[allow(dead_code)]
pub fn serialize<T, S>(value: T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    let json = serde_json::to_string(&value).map_err(S::Error::custom)?;
    serializer.serialize_str(&json)
}

/// Read a string from the input and deserialize it as a `T`.
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: DeserializeOwned,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(D::Error::custom)
}
