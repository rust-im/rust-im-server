use crate::errors::Errors;
use crate::jwt_auth::Auth;
use crate::services::{self, Db};

use rocket::serde::json::{json, Json, Value};
use rocket::Route;
use serde::Deserialize;

pub fn routes() -> Vec<Route> {
    routes![get_users_info]
}

#[derive(Debug, Deserialize)]
pub struct ReqGetUsersInfo {
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
                Errors::new(&[("db_err", "db_err")])
            })
    })
    .await
}
