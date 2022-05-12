use crate::jwt_auth::Auth;
use crate::config::AppState;
use crate::errors::{Errors, FieldValidator};
use crate::services::{self, Db};

use rocket::serde::json::{json, Json, Value};
use rocket::{State, Route};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUser {
    user: NewUserData,
}

#[derive(Deserialize, Validate)]
struct NewUserData {
    #[validate(length(min = 1))]
    username: Option<String>,
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = 8))]
    password: Option<String>,
}

pub fn routes() -> Vec<Route> {
    routes![post_users]
}

#[post("/users", format = "json", data = "<new_user>")]
pub async fn post_users(
    new_user: Json<NewUser>,
    db: Db,
    state: &State<AppState>,
) -> Result<Value, Errors> {
   Ok(json!({}))
}
