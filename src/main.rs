use recs;




#[rocket::main]
async fn main() {
    // let result = rocket::build().launch().await;
    let result = recs::rocket().launch().await;
}