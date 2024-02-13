extern crate dotenv;
use std::{
    env,
    io::{Error, ErrorKind},
};

use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::models::db_models::{UpdatedUser, User};

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = match Client::with_uri_str(uri).await {
            Ok(client) => client,
            Err(_) => panic!("Error connecting to database"),
        };

        let db = client.database("rust-api");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            ..new_user
        };

        let user = match self.col.insert_one(new_doc, None).await.ok() {
            Some(user) => user,
            None => return Err(Error::new(ErrorKind::InvalidInput, "Error creating user")), // or handle the error as you see fit
        };

        Ok(user)
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = match ObjectId::parse_str(id) {
            Ok(obj_id) => obj_id,
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid ID")),
        };
        let filter = doc! {"_id": obj_id};
        let user_detail = match self.col.find_one(filter, None).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(Error::new(ErrorKind::NotFound, "User not found")),
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Error getting user")),
        };

        Ok(user_detail)
    }

    pub async fn update_user(
        &self,
        id: &String,
        new_user: UpdatedUser,
    ) -> Result<UpdateResult, Error> {
        let obj_id = match ObjectId::parse_str(id) {
            Ok(obj_id) => obj_id,
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid ID")),
        };
        let filter = doc! {"_id": obj_id};

        let mut update_doc = doc! {};
        if let Some(first_name) = new_user.first_name {
            update_doc.insert("first_name", first_name);
        }
        if let Some(last_name) = new_user.last_name {
            update_doc.insert("last_name", last_name);
        }
        if let Some(age) = new_user.age {
            update_doc.insert("age", age);
        }
        if let Some(pensum) = new_user.pensum {
            update_doc.insert("pensum", pensum);
        }
        if let Some(location) = new_user.location {
            update_doc.insert("location", location);
        }
        if let Some(occupation) = new_user.occupation {
            update_doc.insert("occupation", occupation);
        }
        if let Some(ahv_nr) = new_user.ahv_nr {
            update_doc.insert("ahv_nr", ahv_nr);
        }

        let update = doc! {"$set": update_doc};
        let updated_doc = match self.col.update_one(filter, update, None).await.ok() {
            Some(user) => user,
            None => return Err(Error::new(ErrorKind::InvalidInput, "Error updating user")),
        };
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = match ObjectId::parse_str(id) {
            Ok(obj_id) => obj_id,
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid ID")),
        };
        let filter = doc! {"_id": obj_id};
        let user_detail = match self.col.delete_one(filter, None).await.ok() {
            Some(user) => user,
            None => return Err(Error::new(ErrorKind::InvalidInput, "Error deleting user")),
        };

        Ok(user_detail)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = match self.col.find(None, None).await.ok() {
            Some(user) => user,
            None => return Err(Error::new(ErrorKind::InvalidInput, "Error getting users")),
        };

        let mut users: Vec<User> = Vec::new();
        while let Some(user) = match cursors.try_next().await.ok() {
            Some(user) => user,
            None => return Err(Error::new(ErrorKind::InvalidInput, "Error getting users")),
        } {
            users.push(user)
        }
        Ok(users)
    }
}
