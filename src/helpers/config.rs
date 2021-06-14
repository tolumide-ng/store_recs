use rocket::fairing::AdHoc;
use rocket::Config;
use rocket_sync_db_pools::{database, diesel};
use std::collections::HashMap;
use std::env;

/// Create rocket config from env variables
pub fn from_env() -> Config {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    match env::var("DATABASE_URL") {
        Ok(val) => {
            database_config.insert("url", val);
            databases.insert("diesel_postgres_pool", database_config);
        }
        Err(e) => {
            panic!("NO DATABASE_URL found in environment variable {}", e);
        }
    };

    let localhost = std::net::Ipv4Addr::new(127, 0, 0, 1);

    Config {
        port: 8080,
        // address: localhost,
        ..Default::default()
    }
}

#[database("recs_users")]
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
