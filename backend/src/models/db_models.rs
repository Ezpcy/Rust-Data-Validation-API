use crate::library::lib::{validate_update, validate_user};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::xml_models::Config;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub age: String,
    pub pensum: String,
    pub location: String,
    pub occupation: String,
    pub ahv_nr: String,
}

impl User {
    pub fn new(
        config: Config,
        first_name: String,
        last_name: String,
        age: String,
        pensum: String,
        location: String,
        occupation: String,
        ahv_nr: String,
    ) -> Result<Self, String> {
        validate_user(
            config,
            &first_name,
            &last_name,
            &age,
            &pensum,
            &location,
            &occupation,
            &ahv_nr,
        )?;
        Ok(User {
            id: None,
            first_name,
            last_name,
            age,
            pensum,
            location,
            occupation,
            ahv_nr,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatedUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub age: Option<String>,
    pub pensum: Option<String>,
    pub location: Option<String>,
    pub occupation: Option<String>,
    pub ahv_nr: Option<String>,
}

impl UpdatedUser {
    pub fn new(
        config: Config,
        first_name: Option<String>,
        last_name: Option<String>,
        age: Option<String>,
        pensum: Option<String>,
        location: Option<String>,
        occupation: Option<String>,
        ahv_nr: Option<String>,
    ) -> Result<Self, String> {
        validate_update(
            config,
            &first_name,
            &last_name,
            &age,
            &pensum,
            &location,
            &occupation,
            &ahv_nr,
        )?;
        Ok(UpdatedUser {
            first_name,
            last_name,
            age,
            pensum,
            location,
            occupation,
            ahv_nr,
        })
    }
}

/// # Summary
/// Used for String responses.
/// # Examples
///
/// ```
/// use crate::models::models::MSG;
/// let msg = MSG {
///    message: "User successfully created!".to_string(),
/// };
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct MSG {
    pub message: String,
}
