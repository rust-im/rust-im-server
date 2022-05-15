use crate::errors::Errors;
use crate::jwt_auth::Auth;
use crate::services::{self, Db};

use rocket::serde::json::{json, Json, Value};
use rocket::Route;
use serde::Deserialize;

pub fn routes() -> Vec<Route> {
    routes![get_users_info, get_self_user_info]
}

#[derive(Debug, Deserialize)]
pub struct ReqGetUsersInfo {
    #[serde(rename = "operationID")]
    _operation_id: Option<String>,
    #[serde(rename = "userIDList")]
    user_id_list: Option<Vec<String>>,
}

#[post("/user/get_users_info", format = "json", data = "<get_users>")]
pub async fn get_users_info(
    db: Db,
    auth: Auth,
    get_users: Json<ReqGetUsersInfo>,
) -> Result<Value, Errors> {
    let get_users = get_users.into_inner();
    let user_id_list = get_users.user_id_list.unwrap_or_default();
    let _op_user_id = auth.user_id;
    db.run(move |conn| {
        services::users::get_user_by_user_ids(conn, user_id_list)
            .map(|users| json!({ "data": users }))
            .map_err(|err| {
                println!("get_users_info {}", err);
                Errors::new(&[("get_users_info", "db_err")])
            })
    })
    .await
}

#[derive(Debug, Deserialize)]
pub struct ReqEmpty {
    #[serde(rename = "operationID")]
    _operation_id: Option<String>,
}

#[post("/user/get_self_user_info", format = "json", data = "<get_self_info>")]
pub async fn get_self_user_info(
    db: Db,
    auth: Auth,
    get_self_info: Json<ReqEmpty>
) -> Result<Value, Errors> {
    let user_id = auth.user_id;
    let _empty = get_self_info.into_inner();
    db.run(move |conn| {
        services::users::get_user_by_user_id(conn, user_id)
            .map(|user| json!({ "data": user }))
            .map_err(|err| {
                println!("get_users_info {}", err);
                Errors::new(&[("get_self_user_info", "db_err")])
            })
    })
    .await
}
