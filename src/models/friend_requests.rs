use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct FriendRequest {
    pub from_user_id: String,
    pub to_user_id: String,
    pub handle_result: i32,
    pub req_msg: String,
    pub create_time: DateTime<Utc>,
    pub handler_user_id: String,
    pub handle_msg: String,
    pub handle_time: DateTime<Utc>,
    pub ex: String,
    #[serde(skip_serializing)]
    pub is_deleted: bool,
}
