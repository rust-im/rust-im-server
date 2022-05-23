use crate::constants;
use chrono::{DateTime, Utc};
use md5;
use rand::Rng;

pub struct DtoNotifyMessage {
    pub send_id: String,
    pub recv_id: String,
    pub content_type: u32,
    pub session_type: u32,
    pub msg_from: u32,
    pub operation_id: String,
    pub content: String,
    pub group_id: String,
}

impl DtoNotifyMessage {
    fn attach(&self, server_msg_id: String, send_time: i64) -> DtoNotifyMessageData {
        DtoNotifyMessageData {
            send_id: self.send_id.clone(),
            recv_id: self.recv_id.clone(),
            content_type: self.content_type,
            session_type: self.session_type,
            msg_from: self.msg_from,
            operation_id: self.operation_id.clone(),
            content: self.content.clone(),
            group_id: self.group_id.clone(),
            server_msg_id,
            send_time,
        }
    }
}

#[derive(Debug)]
pub struct DtoNotifyMessageData {
    pub send_id: String,
    pub recv_id: String,
    pub content_type: u32,
    pub session_type: u32,
    pub msg_from: u32,
    pub operation_id: String,
    pub content: String,
    pub group_id: String,
    pub server_msg_id: String,
    pub send_time: i64,
}

pub fn notify(message: DtoNotifyMessage) {
    let mut message = message;
    match message.session_type {
        constants::GROUP_CHAT_TYPE => {
            message.group_id = message.recv_id;
            message.recv_id = format!("");
        }
        _ => (),
    }

    let server_msg_id = get_msg_id(message.send_id.clone());
    let now: DateTime<Utc> = Utc::now();
    let now = now.timestamp_millis();
    let message = message.attach(server_msg_id, now);
    println!("{:#?}", message);
}

pub fn get_msg_id(send_id: String) -> String {
  let now: DateTime<Utc> = Utc::now();
  let now = now.timestamp();
  let mut rng = rand::thread_rng();
  let random = rng.gen::<u16>();

  format!("{:x}", md5::compute(format!("{}-{}-{}", now, send_id, random).as_bytes()))
}