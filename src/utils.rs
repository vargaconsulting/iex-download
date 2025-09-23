/* This file is part of the IEX2H5 project and is licensed under the MIT License.
 Copyright © 2017–2025 Varga LABS, Toronto, ON, Canada 🇨🇦 Contact: info@vargalabs.com */

use std::fmt;
use serde::de::{self, Visitor};
use serde::Deserializer;

pub fn to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error> where D: Deserializer<'de> {
    struct U64Visitor;

    impl<'de> Visitor<'de> for U64Visitor {
        type Value = u64;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("an integer or a string containing an integer")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            Ok(v)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: de::Error {
            v.parse::<u64>().map_err(E::custom)
        }
    }

    deserializer.deserialize_any(U64Visitor)
}

pub fn to_human(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
