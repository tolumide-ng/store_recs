use crate::validators::auth::NewUser;
use crate::validators::common::{is_email_valid, is_valid_phone, is_valid_string};
use crate::helpers::response_generator::{FailureResponse, SuccessResponse};

pub fn create_new_user(user: NewUser) -> Result<(), FailureResponse> {
    println!("INSIDE HERE>>>");
    let check_email = is_email_valid(user.email);

    if check_email.is_err() {
        println!("BONAGISO, BABY BOO");
        return check_email
    };
    is_valid_phone(user.phone);
    is_valid_string(Some(user.first_name), "first_name");
    is_valid_string(Some(user.last_name), "last_name");
    is_valid_string(Some(user.password), "password");    

    Ok(())
}



// #[derive(Debug)]
// pub enum MyError {
//     ErrorHere
// }

// #[rocket::async_trait]
// impl<'r, 'a> FromRequest<'r> for NewUser<'a> {
//     type Error = MyError;

//     async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
//         /* .. */
//         println!("MADDDDUESUAX");
//         return Outcome::Failure((Status::BadRequest, MyError::ErrorHere))
//     }
// }