use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::serde::{json::Json, Serialize};
// pub struct ApiResponse<R, E>(pub Result<R, E>);

#[derive(Serialize)]
pub struct FailureResponse {
    message: String,
    status_code: u32,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    data: Option<String>,
    status_code: u32,
}

#[derive(Responder)]
#[response(bound = "T: Serialize, E: Responder<'r, 'o>")]
pub enum ResponseBody<T, E> {
    Ok(Json<T>),
    #[response(status = 404)]
    Err(E, ContentType),
}
