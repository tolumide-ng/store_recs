use recs;

#[rocket::main]
async fn main() {
    // let result = rocket::build().launch().await;
    recs::rocket().launch().await;
}


