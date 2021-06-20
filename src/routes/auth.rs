use crate::validators::auth::{NewUser};
// use rocket::serde::{json::Json,};
use rocket::serde::{json::Json, Serialize};
use crate::helpers::response_generator::ResponseBody;
use crate::validators::common::is_email_valid;





#[post("/signup", data="<user>")]
pub fn signup(user: Json<NewUser>) -> &'static str {

    // match serde_json::from_str(user.into_inner()) {
    //     Ok(user) => {
    //         println!("THE USER>>>>>>>> {:#?}", user);
    //     }
    //     Err(e) => {
    //         panic!("ERROR VALIDATING INPUT {:#?}", e);
    //     }
    // };

    let the_user = user.into_inner();
    // println!("THE USER>>>>>>>> {:#?}", user.into_inner());
    the_user.validate_user();


    // is_email_valid(user);
    // println!("THE REQUEST>>>>>>>>>> {:#?}", user);
    "Request made by Tolumide"

}



#[get("/")]
pub fn todo() -> Json<ResponseBody> {
    Json(ResponseBody::new(200,"Hello World".to_owned()))
}
