use serde::Deserialize;
use validator::{Validate, ValidationError};
use rocket::serde::{json::Json, Serialize};
use rocket::http::Status;
use rocket::request::{self, Outcome, Request, FromRequest};


#[derive(Debug, Validate, Deserialize)]
pub struct NewUser<'a> {
    #[validate(required)]
    pub first_name: &'a str,
    #[validate(required)]
    pub last_name: Option<&'a str>,
    #[validate(email)]
    email: &'a str,
    #[validate(phone)]
    phone: Option<&'a str>,
    phone_code: Option<&'a str>,
    // birthday: 
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

pub fn validate_user(user: Json<NewUser>) {

}

// fn talking () {

//     match signup_data.validate() {
//       Ok(_) => (),
//       Err(e) => return e;
//     };
// }
