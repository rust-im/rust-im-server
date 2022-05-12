use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::user::User;
use crate::schema::users;



#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUserDto<'a> {
    pub user_id: &'a str,
    pub nickname: &'a str,
}

pub fn register_user(conn: &PgConnection, new_user: NewUserDto) -> Result<User, Error> {
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)
}
