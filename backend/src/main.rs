mod api;
mod library;
mod models;
mod repository;

use actix_cors::Cors;
use actix_web::{get, http::header, middleware::Logger, web::Data, App, HttpServer, Responder};
use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user};
use models::xml_models::Config;
use repository::mongodb_repo::MongoRepo;
use std::sync::{Arc, Mutex};

#[get("/")]
async fn health() -> impl Responder {
    format!("Server is up and running")
}

/// # The AppState struct
///
/// The AppState struct is used to share data between the different threads
#[derive(Debug, Clone)]
pub struct AppState {
    pub valid_config: Arc<Mutex<Config>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let state = Data::new(AppState {
        valid_config: Arc::new(Mutex::new(Config::default())),
    });

    // File of the XML document with the validations values
    let file = "valid.xml";

    match library::xml_live_reader::read_xml(&file, state.clone()) {
        Ok(_) => {
            dbg!("XML file read successfully");
        }
        Err(e) => {
            dbg!(e);
        }
    }

    actix_rt::spawn(library::xml_live_reader::async_watch(&file, state.clone()));

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:3000/")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ]);
        App::new()
            .app_data(state.clone())
            .app_data(db_data.clone())
            .service(health)
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_all_users)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
