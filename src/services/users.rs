use diesel::pg::expression::dsl::any;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::models::user::{User, PublicUser};
use crate::schema::users;

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct DtoNewUser<'a> {
    pub user_id: &'a str,
    pub nickname: &'a str,
    pub face_url: &'a str,
    pub gender: i32,
    pub phone_number: Option<String>,
    pub birth: i32,
    pub email: Option<String>,
    pub ex: &'a str,
    pub attached_info: &'a str,
}

pub fn register_user(conn: &PgConnection, new_user: DtoNewUser) -> Result<User, Error> {
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)
}

pub fn get_user_by_user_id(conn: &PgConnection, user_id: String) -> Result<User, Error> {
    users::table
        .filter(users::is_deleted.eq(false))
        .filter(users::user_id.eq(user_id))
        .get_result(conn)
}

pub fn get_user_by_user_ids(conn: &PgConnection, user_ids: Vec<String>) -> Result<Vec<PublicUser>, Error> {
    users::table
        .filter(users::is_deleted.eq(false))
        .filter(users::user_id.eq(any(user_ids)))
        .get_results::<User>(conn)
        .map(|res| {
            res.into_iter().map(|u| u.to_public()).collect()
        })
}
