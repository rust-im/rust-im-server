use rocket::config::Config;
use rocket::fairing::AdHoc;
use rocket::figment::Figment;
use std::collections::HashMap;
use std::env;

/// Debug only secret for JWT encoding & decoding.
pub const APP_SECRET: &'static str = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=";
pub const JWT_SECRET: &'static str = "cnVzdGlt"; // base64 rustim

/// js toISOString() in test suit can't handle chrono's default precision
pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

pub const TOKEN_PREFIX: &'static str = "Token ";
pub const REDIS_ADDRESS: &'static str = "redis://127.0.0.1:6379";
pub const WS_ADDRESS: &'static str = "127.0.0.1:54321";


pub struct AppState {
    pub jwt_secret: Vec<u8>,
    pub app_secret: Vec<u8>,
}

impl AppState {
    pub fn get_config() -> AppState {
            // Rocket doesn't expose it's own secret_key, so we use our own here.
            let app_secret = env::var("APP_SECRET_KEY").unwrap_or_else(|err| {
                if cfg!(debug_assertions) {
                    APP_SECRET.to_string()
                } else {
                    panic!("No SECRET_KEY environment variable found: {:?}", err)
                }
            });

            let jwt_secret = env::var("JWT_SECRET_KEY").unwrap_or_else(|err| {
                if cfg!(debug_assertions) {
                    JWT_SECRET.to_string()
                } else {
                    panic!("No SECRET_KEY environment variable found: {:?}", err)
                }
            });

            AppState {
                jwt_secret: jwt_secret.into_bytes(),
                app_secret: app_secret.into_bytes(),
            }
    }

    pub fn manage() -> AdHoc {
        AdHoc::on_ignite("Manage config", |rocket| async move {
            rocket.manage(AppState::get_config())
        })
    }
}

/// Create rocket config from environment variables
pub fn from_env() -> Figment {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    let database_url =
        env::var("DATABASE_URL").expect("No DATABASE_URL environment variable found");
    database_config.insert("url", database_url);
    databases.insert("diesel_postgres_pool", database_config);

    Config::figment()
        .merge(("port", port))
        .merge(("databases", databases))
}
