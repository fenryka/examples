#[allow(dead_code)]
#[derive(Default)]
pub struct KatDuration {
    secs: u64,
    nanos: u32,
}

use std;
use std::fmt;
use dotenv::dotenv;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};

impl fmt::Display for KatDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "secs: {}, nanos: {}", self.secs, self.nanos)
    }
}

impl<'de> Deserialize<'de> for KatDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = KatDuration;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_map<V>(self, mut map: V) -> Result<KatDuration, V::Error>
                where
                    V: MapAccess<'de>,
            {
                let mut kd : KatDuration = Default::default();

                while let Some(kv) = map.next_entry::<String, String>()? {
                    match kv.0.as_str() {
                        "secs" => {
                            kd.secs = kv.1.parse::<u64>().unwrap();
                        }
                        "nanos" => {
                            kd.nanos = kv.1.parse::<u32>().unwrap();
                        }
                        _ => {

                        }
                    }
                }

                Ok(kd)
            }
        }

        const FIELDS: &[&str] = &["secs", "nanos"];
        deserializer.deserialize_struct("Duration", FIELDS, DurationVisitor)
    }
}





fn main() {
    dotenv().expect("Failed to read .env file");

    let d = envy::from_env::<KatDuration>().unwrap_or_else(|error|{
        println!("Failed to parse env-vars, {}", error);
        std::process::exit(1)
    });

    println!("Duration: {}", d)
}
