use serde::{Deserialize, Deserializer, Serialize, Serializer};

fn deserialize_comma_list<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(String::deserialize(deserializer)?
        .split(",")
        .filter_map(|string| match string.len() {
            0 => None,
            _ => Some(String::from(string)),
        })
        .collect())
}

fn serialize_comma_list<S>(v: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(v.join(",").as_str())
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Filters {
    #[serde(default)]
    #[serde(
        deserialize_with = "deserialize_comma_list",
        serialize_with = "serialize_comma_list"
    )]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub restrictions: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
}
