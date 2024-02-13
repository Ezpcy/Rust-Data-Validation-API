//! This module contains the endpoints
use crate::{
    models::db_models::{UpdatedUser, User, MSG},
    repository::mongodb_repo::MongoRepo,
    AppState,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

/// **POST /user** Endpoint to create a new user
/// # Arguments
///
/// * `db: Data<MongoRepo>` - The database connection
/// * `new_user: Json<User>` - The user data from the request body
///
/// # Returns
///
/// * `HttpResponse` - The response with the status code and a message
#[post("/user")]
pub async fn create_user(
    db: Data<MongoRepo>,
    new_user: Json<User>,
    state: Data<AppState>,
) -> HttpResponse {
    let guard = state.valid_config.lock();
    let config = match guard {
        // get the config from the state it does not have clone trait
        Ok(guard) => {
            let config = guard;
            config
        }
        Err(e) => {
            dbg!(e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    // Create a new user struct from the request data and validate it with the validate_user function from the middleware
    let data = User::new(
        config.clone(),
        new_user.first_name.to_owned(),
        new_user.last_name.to_owned(),
        new_user.age.to_owned(),
        new_user.pensum.to_owned(),
        new_user.location.to_owned(),
        new_user.occupation.to_owned(),
        new_user.ahv_nr.to_owned(),
    );

    match data {
        Ok(user) => {
            let result = db.create_user(user).await;
            match result {
                Ok(user) => HttpResponse::Ok().json(
                    serde_json::json!({"message" : "User successfully created!", "id" : user.inserted_id.as_object_id()}),
                ),
                Err(_) => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(error_message) => {
            let response = MSG {
                message: error_message,
            };
            HttpResponse::BadRequest().json(response)
        }
    }
}

/// **GET /user/{id}** Endpoint to get a user by id
///
/// # Arguments
/// * `db: Data<MongoRepo>` - The database connection
/// * `path: Path<String>` - The id of the user
///
/// # Returns
///
/// * `HttpResponse` - The response with the status code and a message of the retrieved user
#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user_detail = db.get_user(&id).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// **PUT /user/{id}** Endpoint to update a user by id
///
/// # Arguments
/// * `db: Data<MongoRepo>` - The database connection
/// * `path: Path<String>` - The id of the user
/// * `new_user: Json<UpdatedUser>` - The user data from the request body
/// # Returns
/// * `HttpResponse` - The response with the status code and a message
#[put("/user/{id}")]
pub async fn update_user(
    state: Data<AppState>,
    db: Data<MongoRepo>,
    path: Path<String>,
    new_user: Json<UpdatedUser>,
) -> HttpResponse {
    let guard = state.valid_config.lock();
    let config = match guard {
        // get the config from the state it does not have clone trait
        Ok(guard) => {
            let config = guard;
            config
        }
        Err(e) => {
            dbg!(e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let id = path.into_inner();
    if id.is_empty() {
        return {
            let response = MSG {
                message: "invalid ID".to_string(),
            };
            HttpResponse::BadRequest().json(response)
        };
    };
    let data = match UpdatedUser::new(
        config.clone(),
        new_user.first_name.to_owned(),
        new_user.last_name.to_owned(),
        new_user.age.to_owned(),
        new_user.pensum.to_owned(),
        new_user.location.to_owned(),
        new_user.occupation.to_owned(),
        new_user.ahv_nr.to_owned(),
    ) {
        Ok(data) => data,
        Err(err) => {
            return {
                let response = MSG { message: err };
                HttpResponse::BadRequest().json(response)
            }
        }
    };

    let update_result = db.update_user(&id, data).await;

    match update_result {
        Ok(res) => {
            if res.modified_count == 1 {
                return {
                    let response = MSG {
                        message: "User successfully updated!".to_string(),
                    };
                    HttpResponse::Ok().json(response)
                };
            } else {
                return {
                    let response = MSG {
                        message: "Nothing changed.".to_string(),
                    };
                    HttpResponse::NotFound().json(response)
                };
            }
        }
        Err(err) => {
            let response = MSG {
                message: err.to_string(),
            };
            HttpResponse::BadRequest().json(response)
        }
    }
}

/// **DELETE /user/{id}** Endpoint to delete a user by id
///
/// # Arguments
/// * `db: Data<MongoRepo>` - The database connection
/// * `path: Path<String>` - The id of the user
/// # Returns
/// * `HttpResponse` - The response with the status code and a message
#[delete("/user/{id}")]
pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_user(&id).await;

    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return {
                    let response = MSG {
                        message: "User successfully deleted!".to_string(),
                    };
                    HttpResponse::Ok().json(response)
                };
            } else {
                return {
                    let response = MSG {
                        message: "User with specified ID not found!".to_string(),
                    };
                    HttpResponse::NotFound().json(response)
                };
            }
        }
        Err(err) => {
            let response = MSG {
                message: err.to_string(),
            };
            HttpResponse::BadRequest().json(response)
        }
    }
}

/// **GET /user/{id}** Endpoint to get all users
///
/// # Arguments
/// * `db: Data<MongoRepo>` - The database connection
/// # Returns
/// * `HttpResponse` - The response with the status code and a message
#[get("/users")]
pub async fn get_all_users(db: Data<MongoRepo>) -> HttpResponse {
    let users = db.get_all_users().await;

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
