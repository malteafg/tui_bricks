use serde::{
    Deserialize, Deserializer,
    de::{Error, Unexpected},
};

pub fn bool_deserializer<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match s {
        "True" | "true" => Ok(true),
        "False" | "false" => Ok(false),
        _ => Err(Error::invalid_value(
            Unexpected::Str(s),
            &"True/true or False/false",
        )),
    }
}
