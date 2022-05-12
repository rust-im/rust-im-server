use rocket::routes;
use rocket::serde::json::{json, Json, Value};
use rocket::Route;
use serde::{Deserialize, Serialize};

use crate::errors::{Errors, FieldValidator};
use crate::services::users::NewUserDto;
use crate::services::{self, Db};

pub fn routes() -> Vec<Route> {
    routes![register_user]
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUserData {
    pub secret: Option<String>,
    pub platform: Option<u8>,
    #[validate(length(min = 1))]
    #[serde(rename = "userID")]
    pub user_id: Option<String>,
    #[validate(length(min = 1))]
    pub nickname: Option<String>,
    #[serde(rename = "faceUrl")]
    pub face_url: Option<String>,
    pub gender: Option<i32>,
    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,
    pub birth: Option<u32>,
    pub email: Option<String>,
    pub ex: Option<String>,
    #[serde(rename = "attachInfo")]
    pub attach_info: Option<String>,
    #[serde(rename = "operationID")]
    pub operation_id: Option<String>,
}

#[post("/auth/user_register", format = "json", data = "<register_user>")]
pub async fn register_user(db: Db, register_user: Json<RegisterUserData>) -> Result<Value, Errors> {
    let register_user = register_user.into_inner();

    let mut extractor = FieldValidator::validate(&register_user);

    let user_id = extractor.extract("user_id", register_user.user_id);
    let nickname = extractor.extract("nickname", register_user.nickname);

    let face_url = register_user.face_url.unwrap_or_default();
    let gender = register_user.gender.unwrap_or_default();
    let phone_number = register_user.phone_number.unwrap_or_default();
    let birth = register_user.birth.unwrap_or_default();
    let email = register_user.email.unwrap_or_default();
    let ex = register_user.ex.unwrap_or_default();
    let attached_info = register_user.attach_info.unwrap_or_default();

    extractor.check()?;

    db.run(move |conn| {
        let register_user = NewUserDto {
            user_id: &user_id,
            nickname: &nickname,
        };
        println!("register_user {:#?}", &register_user);
        services::users::register_user(conn, register_user)
            .map(|user| json!({ "data": user }))
            .map_err(|err| {
                println!("register_user err: {}", err);
                Errors::new(&[("db_error", "register error")])
            })
    })
    .await
}
