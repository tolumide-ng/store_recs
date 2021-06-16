pub mod helpers;
pub mod schema;
// pub mod models;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate uuid;
use diesel::result::Error;
use dotenv::dotenv;
// use helpers::response_body::{FailureResponse, ResponseBody, SuccessResponse};
use helpers::config::{db_config, AppState, RecsDbConn};
use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};
use rocket_cors::{Cors, CorsOptions};

#[derive(Serialize)]
struct ResponseBody {
    status_code: u32,
    message: String,
}

#[get("/")]
fn todo() -> Json<ResponseBody> {
    Json(ResponseBody {
        status_code: 200,
        message: "Hello World".to_owned(),
    })
}

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
    Json(ResponseBody {
        status_code: 404,
        message: String::from("Resource Not Found"),
    })
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
        .mount("/", routes![todo])
        // .attach(cors_fairing())
        .attach(RecsDbConn::fairing())
        .attach(AppState::manage())
        .register("/", catchers![not_found, internal_error])
}
