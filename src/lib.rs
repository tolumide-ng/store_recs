pub mod helpers;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
use diesel::result::Error;
use dotenv::dotenv;
// use helpers::response_body::{FailureResponse, ResponseBody, SuccessResponse};
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_cors::Cors;

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("Error Creating Cors Fairing")
}

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

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[catch(404)]
fn not_found() -> Json<ResponseBody> {
    Json(ResponseBody {
        status_code: 404,
        message: String::from("Resource Not Found"),
    })
}

#[launch]
pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    // rocket::custom()
}
