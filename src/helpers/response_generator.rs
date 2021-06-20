use rocket::serde::{json::Json, Serialize};
use std::collections::HashMap;


#[derive(Serialize)]
pub struct ResponseBody {
    status_code: u32,
    message: String,
}

impl ResponseBody {
    pub fn new (status_code: u32, message: String) -> Self {
        ResponseBody {status_code, message}
    }
}


pub trait ResponseTrait {
    fn modify_response() {}
}


#[derive(Serialize)]
pub struct SuccessResponse<T: ResponseTrait> {
    data: T,
    status_code: u32,
}

impl<T> SuccessResponse<T> where T: ResponseTrait {
    pub fn new(status_code: u32, data: T) -> Self {
        SuccessResponse{
            status_code,
            data,
        }
    }
}



pub enum ApiResponse<T, E> 
where T: ResponseTrait, E: ResponseTrait {
    Ok(T),
    Error(E)
}


#[derive(Serialize)]
pub struct FailureResponse{
    status_code: u32,
    message: String,
    // error: Option<T>,
}

impl FailureResponse {
    pub fn new(status_code: u32, message: String) -> Self {
        FailureResponse { status_code, message}
    }
}


impl ResponseTrait for FailureResponse {}


