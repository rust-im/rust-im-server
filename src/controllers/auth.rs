use rocket::Route;
use rocket::serde::json::{json, Json, Value};
use rocket::routes;
use serde::{Deserialize, Serialize};

pub fn routes() -> Vec<Route> {
    routes![register_user]
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUserInfo {
    nickname: Option<String>,
    face_url: Option<String>,
    gender: Option<i8>,
    phone_number: Option<String>,
    birth: Option<u32>,
    email: Option<String>,
    ex: Option<String>,
    app_manager_level: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUserData {
    user_info: RegisterUserInfo,
    operation_id: Option<String>
}

#[post("/auth/user_register", format = "json", data = "<register_user>")]
pub fn register_user(
    register_user: Json<RegisterUserData>,
) -> Value {
    let register_user = register_user.into_inner();
    println!("register_user {:#?}", &register_user);
    json!({})
}