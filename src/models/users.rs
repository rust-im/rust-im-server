use crate::jwt_auth::Auth;
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    #[serde(rename = "userID")]
    pub user_id: String,
    pub nickname: String,
    #[serde(rename = "faceUrl")]
    pub face_url: String,
    pub gender: i32,
    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,
    pub birth: i32,
    pub email: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub app_manager_level: Option<i32>,
    pub ex: String,
    #[serde(rename = "attachInfo")]
    pub attached_info: String,
    #[serde(skip_serializing)]
    pub is_deleted: bool,
}

#[derive(Serialize)]
pub struct PublicUser {
    #[serde(rename = "userID")]
    pub user_id: String,
    pub nickname: String,
    #[serde(rename = "faceUrl")]
    pub face_url: String,
    pub gender: i32,
    pub ex: String,
}

#[derive(Serialize)]
pub struct UserAuth<'a> {
    user_id: &'a str,
    token: String,
    exp: i64,
}

#[derive(Serialize)]
pub struct Profile {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

impl User {
    pub fn to_user_auth(&self, secret: &[u8]) -> UserAuth {
        let exp = Utc::now() + Duration::days(60); // TODO: move to config
        let token = Auth {
            user_id: self.user_id.clone(),
            exp: exp.timestamp(),
        }
        .token(secret);

        UserAuth {
            user_id: &self.user_id,
            token,
            exp: exp.timestamp(),
        }
    }

    pub fn to_public(&self) -> PublicUser {
        PublicUser {
            user_id: self.user_id.clone(),
            nickname: self.nickname.clone(),
            face_url: self.face_url.clone(),
            gender: self.gender,
            ex: self.ex.clone(),
        }
    }
}
