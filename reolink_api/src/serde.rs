//! Deserialization utilities

// Note: many of these are provided by the serde_with crate, consider using it if more complex use cases appear.

use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serializer};
use serde::de::Error;

pub mod bool_as_number {
    use super::*;

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
        let number: u8 = Deserialize::deserialize(deserializer)?;
        Ok(number > 0)
    }

    pub fn serialize<S: Serializer>(v: &bool, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(if *v { 1 } else { 0 })
    }
}

pub mod from_str {
    use super::*;
    use std::borrow::Cow;

    pub fn deserialize<'de, D, T: FromStr>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T::Err: Display,
    {
        let txt: Cow<'de, str> = Deserialize::deserialize(deserializer)?;
        T::from_str(txt.as_ref()).map_err(Error::custom)
    }

    // pub fn serialize<S: Serializer, T:ToString>(v: &T, serializer: S) -> Result<S::Ok, S::Error> {
    //     serializer.serialize_str(&v.to_string())
    // }
}
