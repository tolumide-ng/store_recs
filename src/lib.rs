#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate uuid;
extern crate validator;
use diesel::result::Error;
use dotenv::dotenv;
use serde_json::json;

#[macro_use]
extern crate validator_derive;


pub mod helpers;
pub mod schema;
pub mod models;
pub mod validators;
pub mod routes;
pub mod middlewares;

use helpers::config::{db_config, AppState, RecsDbConn};
use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};
use rocket_cors::{Cors, CorsOptions};
use crate::helpers::response_generator::ResponseBody;



pub fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found() -> Json<ResponseBody> {
    Json(ResponseBody::new(404, String::from("Resource Not Found")))
}


pub fn cors_fairing() -> Cors {
    Cors::from_options(&CorsOptions::default()).expect("Cors Fairing cannot be created")
}

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    let db_config = db_config();
    // println!("RECEIVED CONFIG----------------------------------- {:#?}", db_config);

    rocket::custom(db_config)
        .mount("/api", routes![routes::auth::signup, routes::auth::todo])
        // .attach(cors_fairing())
        .attach(RecsDbConn::fairing())
        .attach(AppState::manage())
        .register("/", catchers![not_found, internal_error])
}
