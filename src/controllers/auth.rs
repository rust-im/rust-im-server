use rocket::routes;
use rocket::serde::json::{json, Json, Value};
use rocket::Route;
use serde::Deserialize;

use crate::errors::{Errors, FieldValidator};
use crate::services::users::NewUserDto;
use crate::services::{self, Db};
use crate::config::JWT_SECRET;

pub fn routes() -> Vec<Route> {
    routes![register_user, user_token]
}

#[derive(Debug, Deserialize, Validate)]
pub struct ReqRegisterUserData {
    _secret: Option<String>,
    _platform: Option<u8>,
    #[validate(length(min = 1))]
    #[serde(rename = "userID")]
    user_id: Option<String>,
    #[validate(length(min = 1))]
    nickname: Option<String>,
    #[serde(rename = "faceUrl")]
    face_url: Option<String>,
    gender: Option<i32>,
    #[serde(rename = "phoneNumber")]
    phone_number: Option<String>,
    birth: Option<i32>,
    #[validate(email)]
    email: Option<String>,
    ex: Option<String>,
    #[serde(rename = "attachInfo")]
    attach_info: Option<String>,
    #[serde(rename = "operationID")]
    _operation_id: Option<String>,
}

#[post("/auth/user_register", format = "json", data = "<register_user>")]
pub async fn register_user(
    db: Db,
    register_user: Json<ReqRegisterUserData>,
) -> Result<Value, Errors> {
    let register_user = register_user.into_inner();

    let mut extractor = FieldValidator::validate(&register_user);

    let user_id = extractor.extract("user_id", register_user.user_id);
    let nickname = extractor.extract("nickname", register_user.nickname);

    let face_url = register_user.face_url.unwrap_or_default();
    let gender = register_user.gender.unwrap_or_default();
    let phone_number = register_user.phone_number;
    let birth = register_user.birth.unwrap_or_default();
    let email = register_user.email;
    let ex = register_user.ex.unwrap_or_default();
    let attached_info = register_user.attach_info.unwrap_or_default();

    extractor.check()?;

    db.run(move |conn| {
        let register_user = NewUserDto {
            user_id: &user_id,
            nickname: &nickname,
            face_url: &face_url,
            gender,
            phone_number,
            birth,
            email,
            ex: &ex,
            attached_info: &attached_info,
        };
        println!("register_user {:#?}", &register_user);
        services::users::register_user(conn, register_user)
            .map(|user| json!({ "data": user }))
            .map_err(|err| {
                println!("auth/register_user err: {}", err);
                Errors::new(&[("db_error", "register error")])
            })
    })
    .await
}

#[derive(Debug, Deserialize, Validate)]
pub struct ReqUserTokenData {
    _secret: Option<String>,
    #[serde(rename = "userID")]
    user_id: Option<String>,
    #[serde(rename = "operationID")]
    _operation_id: Option<String>,
    _platform: Option<u8>,
}

#[post("/auth/user_token", format = "json", data = "<user_token_data>")]
pub async fn user_token(db: Db, user_token_data: Json<ReqUserTokenData>) -> Result<Value, Errors> {
    let user_token_data = user_token_data.into_inner();
    let mut extractor = FieldValidator::validate(&user_token_data);

    let user_id = extractor.extract("user_id", user_token_data.user_id);
    extractor.check()?;
    db.run(move |conn| {
        services::users::get_user_by_user_id(conn, user_id)
            .map(|user| {
                let jwt_token_auth = user.to_user_auth(JWT_SECRET.as_bytes());
                json!({ "data": jwt_token_auth })
            })
            .map_err(|err| {
                println!("auth/user_token err: {}", err);
                Errors::new(&[("db_error", "auth/user_token error")])
            })
    })
    .await
}
