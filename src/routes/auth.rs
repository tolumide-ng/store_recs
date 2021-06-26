use crate::validators::auth::{NewUser};
// use rocket::serde::{json::Json,};
use rocket::serde::{json::Json, Serialize};
use crate::helpers::response_generator::ResponseBody;
// use crate::validators::common::is_email_valid;
use crate::middlewares::auth::create_new_user;


use rocket::http::Status;
use rocket::request::{self, Outcome, Request, FromRequest};

struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for NewUser<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {

            key == "valid_api_key"
        }


        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            // Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}



use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::http::{ContentType, RawStr};



#[derive(Debug)]
pub enum Error {
    TooLarge,
    NoColon,
    InvalidAge,
    Io(std::io::Error),
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewUser<'r> {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;
        use rocket::outcome::Outcome::*;

        // Ensure the content type is correct before opening the data.

        println!(">>>>>>>>>>>>>!!!!!!!!!!>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

        println!(">>>>>>>>!!!!!!!!!!!!!!!>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        

        // let person_ct = ContentType::new("application", "x-person");
        // if req.content_type() != Some(&person_ct) {
        //     return Forward(data);
        // }

        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");

        // Use a configured limit with name 'person' or fallback to default.
        let limit = req.limits().get("person").unwrap_or(256.bytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => {
                println!("we have a value");
                string.into_inner()
            },
            Ok(_) => {
                println!("  I DON'T KNOW WHAT THIS IS >>>>>>>>>>>");
                // return Failure((Status::PayloadTooLarge, TooLarge))
            },
            Err(e) => {
                println!("AN ERROR>>>>>>>>>>> {:#?}", e);
                // return Failure((Status::InternalServerError, Io(e)))
            },
        };


        println!("IT WENT THROUGH FINE>>>>>>>>>>>>>");

        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);


        println!(">>>>>>>!!!!!!!!!!!!!!!!!!!>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> {:#?}", string);

        // Split the string into two pieces at ':'.
        let (name, age) = match string.find(':') {
            Some(i) => (&string[..i], &string[(i + 1)..]),
            None => return Failure((Status::UnprocessableEntity, NoColon)),
        };

        // Parse the age.
        let age: u16 = match age.parse() {
            Ok(age) => age,
            Err(_) => return Failure((Status::UnprocessableEntity, InvalidAge)),
        };

        Success(NewUser {..NewUser::default() })
    }
}




#[post("/signup", data="<user>")]
pub fn signup(user: NewUser) -> &'static str {


    // match serde_json::from_str(user.into_inner()) {
    //     Ok(user) => {
    //         println!("THE USER>>>>>>>> {:#?}", user);
    //     }
    //     Err(e) => {
    //         panic!("ERROR VALIDATING INPUT {:#?}", e);
    //     }
    // };

    // let the_user = user.into_inner();
    // create_new_user(the_user);
    // println!("THE USER>>>>>>>> {:#?}", user.into_inner());
    // the_user.validate_user();


    // is_email_valid(user);
    // println!("THE REQUEST>>>>>>>>>> {:#?}", user);
    "Request made by Tolumide"

}



#[get("/")]
pub fn todo() -> Json<ResponseBody> {
    Json(ResponseBody::new(200,"Hello World".to_owned()))
}
