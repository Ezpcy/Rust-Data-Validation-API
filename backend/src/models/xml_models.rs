use serde::Deserialize;

// ----------------- DefaultSettings -----------------

#[derive(Debug, PartialEq, Default, Deserialize, Clone)]
#[serde(default)]
#[serde(rename = "config")]
pub struct Config {
    #[serde(rename = "defaultSettings")]
    pub default_settings: DefaultSettings,
    #[serde(rename = "validationRules")]
    pub validation_rules: ValidationRules,
}

#[derive(Debug, PartialEq, Default, Deserialize, Clone)]
#[serde(default)]
#[serde(rename = "defaultSettings")]
pub struct DefaultSettings {
    #[serde(rename = "@language")]
    pub language: String,
}

#[derive(Debug, PartialEq, Default, Deserialize, Clone)]
#[serde(default)]
#[serde(rename = "validationRules")]
pub struct ValidationRules {
    pub person: Person,
}

#[derive(Debug, PartialEq, Default, Deserialize, Clone)]
#[serde(default, rename = "person")]
pub struct Person {
    #[serde(rename = "name")]
    pub name: Name,
    #[serde(rename = "age")]
    pub age: Age,
    #[serde(rename = "email")]
    pub email: Email,
    #[serde(rename = "pensum")]
    pub pensum: Pensum,
}

#[derive(Debug, PartialEq, Default, Deserialize, Clone)]
#[serde(default)]
#[serde(rename = "name")]
pub struct Name {
    #[serde(rename = "@type")]
    pub type_: String,
    #[serde(rename = "@min")]
    pub min: u32,
    #[serde(rename = "@max")]
    pub max: u32,
}

#[derive(Debug, PartialEq, Default, Deserialize, Clone)]
#[serde(default)]
#[serde(rename = "age")]
pub struct Age {
    #[serde(rename = "@type")]
    pub type_: String,
    #[serde(rename = "@minValue")]
    pub min: u32,
    #[serde(rename = "@maxValue")]
    pub max: u32,
}

#[derive(Debug, PartialEq, Default, Deserialize, Clone)]
#[serde(default)]
#[serde(rename = "email")]
pub struct Email {
    #[serde(rename = "@type")]
    pub type_: String,
}

#[derive(Debug, PartialEq, Default, Deserialize, Clone)]
#[serde(default)]
#[serde(rename = "pensum")]
pub struct Pensum {
    #[serde(rename = "@type")]
    pub type_: String,
    #[serde(rename = "@minValue")]
    pub min_value: u32,
    #[serde(rename = "@maxValue")]
    pub max_value: u32,
    #[serde(rename = "@default")]
    pub default: u32,
}
