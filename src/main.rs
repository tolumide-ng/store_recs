use recs;

#[rocket::main]
async fn main() {
    // let result = rocket::build().launch().await;
    match recs::rocket().launch().await {
        Ok(_value) => {}
        Err(_e) => {}
    };
}


