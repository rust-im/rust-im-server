use crate::constants;
use crate::messages::send_message::{self, DtoNotifyMessage};

use rocket::serde::json::json;

pub fn friend_apply_notification(
  from_user_id: String,
  to_user_id: String,
  operation_id: String
) {
    let content_type = constants::FRIEND_APPLICATION_NOTIFICATION;
    let content = json!({
      "tips": {
        "defaultTips": "hello"
      }
    })
    .to_string();
    friend_notify(from_user_id, to_user_id, operation_id, content_type, content);
}

fn friend_notify(
    send_id: String,
    recv_id: String,
    operation_id: String,
    content_type: u32,
    content: String,
) {
    let message = DtoNotifyMessage {
        send_id,
        recv_id,
        content_type,
        session_type: constants::SINGLE_CHAT_TYPE,
        msg_from: constants::SYSTEM_MESSAGE_TYPE,
        operation_id,
        content,
        group_id: format!(""),
    };

    send_message::notify(message);
}
