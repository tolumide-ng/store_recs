use rocket::fairing::AdHoc;
use rocket::figment::{
    util::map,
    value::{Map, Value},
    Figment,
};
use rocket_sync_db_pools::{database, diesel};
use std::env;

#[database("recs_db")]
pub struct RecsDbConn(diesel::PgConnection);

pub struct AppState {
    pub secret: Vec<u8>,
}

impl AppState {
    pub fn manage() -> AdHoc {
        AdHoc::on_ignite("Manage config", |rocket| async {
            let secret = match env::var("SECRET_KEY") {
                Ok(value) => value,
                Err(e) => {
                    panic!("No SECRET_KEY environment variable found {}", e)
                }
            };

            rocket.manage(AppState {
                secret: secret.into_bytes(),
            })
        })
    }
}

pub fn db_config<'a>() -> Figment {
    match env::var("DATABASE_URL") {
        Ok(url) => {

            println!("THE DATABASE URL IS>>>>>>> {:#?}", url);

            let db: Map<_, Value> = map! {
                "url" => Value::from(url),
                "pool_size" => 10.into(),
                "timeout" => 5.into()
            };

            let figment = rocket::Config::figment().merge(("databases", map!["my_db" => db]));

            return figment;
        }
        Err(e) => {
            panic!("NO DATABASE_URL found in environment variable {}", e);
        }
    }
}
