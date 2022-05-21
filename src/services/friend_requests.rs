use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::friend_requests;
use crate::models::friend_requests::FriendRequest;

#[derive(Debug, Insertable)]
#[table_name = "friend_requests"]
pub struct DtoNewFriendRequest<'a> {
  pub from_user_id: &'a str,
  pub to_user_id: &'a str,
  pub req_msg: Option<String>,
}

pub fn new_friend_request(conn: &PgConnection, new_friend_request: DtoNewFriendRequest) -> Result<FriendRequest, Error> {
  diesel::insert_into(friend_requests::table)
      .values(new_friend_request)
      .get_result::<FriendRequest>(conn)
}