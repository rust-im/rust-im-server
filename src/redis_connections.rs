use crate::config::REDIS_ADDRESS;
use redis;
use rocket::fairing::AdHoc;

pub struct RedisClient(redis::Client);

impl RedisClient {
    pub fn get_connection(&self) -> redis::Connection {
        self.0.get_connection().unwrap()
    }

    pub fn manage() -> AdHoc {
        AdHoc::on_ignite("Redis Connect", |rocket| async move {
            let client = redis::Client::open(REDIS_ADDRESS).unwrap();
            rocket.manage(RedisClient(client))
        })
    }
}
