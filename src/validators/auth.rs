use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
struct NewUserData {
    first_name: String,
    last_name: String,
    #[validate(email)]
    email: String,
    phone: Option<String>,
    phone_code: Option<String>
}

