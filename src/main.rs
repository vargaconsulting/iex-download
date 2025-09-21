use reqwest::blocking::Client;
use serde::Deserialize;
use std::time::Duration;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct HistEntry {
    link: String,
    feed: String,
    date: String,
    version: String,
    protocol: String,
    #[serde(deserialize_with = "to_int64")]
    size: u64,
}

fn to_int64<'de, D>(deserializer: D) -> Result<u64, D::Error> where D: serde::Deserializer<'de> {
    struct U64Visitor;

    impl<'de> serde::de::Visitor<'de> for U64Visitor {
        type Value = u64;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("an integer or a string containing an integer")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            Ok(v)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
            v.parse::<u64>().map_err(E::custom)
        }
    }

    deserializer.deserialize_any(U64Visitor)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(60))
        .build()?;

    let url = "https://iextrading.com/api/1.0/hist";
    let resp: HashMap<String, Vec<HistEntry>> = client.get(url).send()?.json()?;

    for (date, entries) in resp {
        for entry in entries {
            println!("{} {} {} {} {} {}",
                date, entry.feed, entry.version, entry.protocol, entry.size, entry.link);
        }
    }

    Ok(())
}
