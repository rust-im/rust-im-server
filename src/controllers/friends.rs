use rocket::Route;

use rocket::serde::json::{json, Json, Value};
use serde::Deserialize;

use crate::errors::Errors;
use crate::services::friend_requests::DtoNewFriendRequest;
use crate::services::{self, Db};

pub fn routes() -> Vec<Route> {
    routes![add_friend]
}

#[derive(Debug, Deserialize)]
pub struct ReqAddFriendInfo {
    #[serde(rename = "commID")]
    comm_id: ReqCommID,
    #[serde(rename = "reqMsg")]
    req_msg: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReqCommID {
    #[serde(rename = "opUserID")]
    _op_user_id: String,
    #[serde(rename = "operationID")]
    _operation_id: Option<String>,
    #[serde(rename = "toUserID")]
    to_user_id: String,
    #[serde(rename = "fromUserID")]
    from_user_id: String,
}

#[post("/friend/add_friend", format = "json", data = "<add_friend_info>")]
pub async fn add_friend(db: Db, add_friend_info: Json<ReqAddFriendInfo>) -> Result<Value, Errors> {
    let add_friend_info = add_friend_info.into_inner();

    db.run(move |conn| {
        let user =
            services::users::get_user_by_user_id(conn, add_friend_info.comm_id.to_user_id.clone());

        if user.is_err() {
            println!("get_user_by_user_id err {:#?}", user.err());
            return Err(Errors::new(&[("db_err", "user_id not exist")]));
        }

        let dto_new_friend_request = DtoNewFriendRequest {
            from_user_id: &add_friend_info.comm_id.from_user_id,
            to_user_id: &add_friend_info.comm_id.to_user_id,
            req_msg: add_friend_info.req_msg,
        };

        let new_friend_request_result =
            services::friend_requests::new_friend_request(conn, dto_new_friend_request);

        if new_friend_request_result.is_err() {
            println!(
                "new_friend_request err {:#?}",
                new_friend_request_result.err()
            );
            return Err(Errors::new(&[("db_err", "insert error")]));
        }

        Ok(json!({ "data": new_friend_request_result.unwrap() }))
    })
    .await
}
