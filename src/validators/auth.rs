use serde::Deserialize;
use validator::{Validate, ValidationError};
use rocket::serde::{json::Json, Serialize};

use rocket::http::Status;
use rocket::request::{self, Outcome, Request, FromRequest};


#[derive(Debug, Validate, Deserialize, Default)]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub password: &'a str,
    // birthday: 
}


#[derive(Debug)]
pub enum MyError {
    ErrorHere
}





impl<'a> NewUser<'a> {
    pub fn validate_user(&self) {
        match self.validate() {
            Ok(user) => {
                println!("THIS IS THE USER'S INFORMATION {:#?}", user);
                // Ok(())
            }
            Err(e) => {
                println!("THERE WAS AN ERROR {:#?}", e);
                panic!("ERROR {:#?}", e);
            }
        };
    }

    // pub fn validate_user(&self) {}
}
