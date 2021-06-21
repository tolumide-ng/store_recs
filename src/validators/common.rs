use rocket::request::{self, Request};
// use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::http::{Status, ContentType};
use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};
use crate::helpers::response_generator::ResponseTrait;
use crate::helpers::response_generator::{FailureResponse, SuccessResponse};


// email
#[derive(Debug, Validate, Deserialize)]
struct Email<'a> {
    #[validate(email)]
    email: &'a str
}

impl<'a> Email<'a> {
    pub fn new (email: &'a str) -> Self {
        Email{email}
    }

    pub fn validate_email(&self) -> Option<validator::ValidationErrors> {
        match self.validate() {
            Ok(user) => {
                None
            }
            Err(e) => {
                Some(e)
            }
        }
    }
}


pub fn is_email_valid<'a>(email: &'a str) -> Result<(), FailureResponse>  {
    let email = Email::new(email);
    if email.validate_email().is_some(){
        println!("THERE WAS AN ERROR------------");
        return Err(FailureResponse::new(400, String::from("Please provide a valid email address")));
    }
    Ok(())
}

pub fn is_valid_string<'a>(input: Option<&'a str>, label: &'a str) -> Result<(), FailureResponse> {
    if input.is_none(){
        return Err(FailureResponse::new(400, format!("Please provide a valid {}", label)));
    }
    Ok(())
}



// phone
#[derive(Debug, Validate, Deserialize)]
struct Phone<'a> {
    #[validate(phone)]
    phone: &'a str
}

impl<'a> Phone<'a> {
    pub fn new (phone: &'a str) -> Self {
        Phone{phone}
    }

    pub fn validate_phone(&self) -> Option<validator::ValidationErrors> {
        match self.validate() {
            Ok(_user) => {
                None
            }
            Err(e) => {
                Some(e)
            }
        }
    }
}

pub fn is_valid_phone<'a>(input: &'a str) -> Result<(), FailureResponse> {
    let phone = Phone::new(input);
    if phone.validate_phone().is_some() {
        return Err(FailureResponse::new(400, String::from("Please provide a valid phone number")));
    }
    Ok(())
}

