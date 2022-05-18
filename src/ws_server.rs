use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::{thread, vec};
use url;
use ws;

use crate::config::AppState;
use crate::config::REDIS_ADDRESS;
use crate::config::WS_ADDRESS;
use crate::jwt_auth;
use redis;

struct WsRouter {
    sender: ws::Sender,
    inner: Box<dyn ws::Handler>,
    redis_client: Rc<redis::Client>,
    ws_conn_map: Rc<RefCell<HashMap<String, Vec<Rc<ws::Sender>>>>>,
}

struct RustIMHandler {
    ws: Rc<ws::Sender>,
    send_id: String,
    platform: u8,
    redis_client: Rc<redis::Client>,
    ws_conn_map: Rc<RefCell<HashMap<String, Vec<Rc<ws::Sender>>>>>,
}

impl ws::Handler for RustIMHandler {
    fn on_open(&mut self, _shake: ws::Handshake) -> ws::Result<()> {
        self.ws_conn_map
            .borrow_mut()
            .entry(self.send_id.clone())
            .or_insert(vec![])
            .push(self.ws.clone());

            println!("{} len {}", self.send_id, self.ws_conn_map.borrow().get(&self.send_id).unwrap().len());
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("{} on_message {:#?}", self.send_id, msg);
        self.ws.send(format!("reply to {}, pong!", self.send_id)).unwrap();

        // let conns = self.ws_conn_map.borrow();
        // if self.send_id == "test1".to_owned() {
        //     let test2_conns = conns.get("test2").unwrap();
        //     for conn in test2_conns.iter() {
        //         conn.send("Hi test2, I'm test1").unwrap();
        //     }
        // } else {
        //     let test1_conns = conns.get("test1").unwrap();
        //     for conn in test1_conns.iter() {
        //         conn.send("Hi test1, i'm test2").unwrap();
        //     }
        // }
        Ok(())
    }

    fn on_close(&mut self, _code: ws::CloseCode, _reason: &str) {
        self.remove_conn();
        println!("{} len {}", self.send_id, self.ws_conn_map.borrow().get(&self.send_id).unwrap().len());
    }

    fn on_error(&mut self, _err: ws::Error) {
        self.remove_conn();
    }
}

impl RustIMHandler {
    fn remove_conn(&mut self) {
        match self.ws_conn_map.borrow_mut().get_mut(&self.send_id) {
            Some(v) => v.retain(|i| i.token() != self.ws.token()),
            None => {}
        }
    }
}

struct NotFound;
impl ws::Handler for NotFound {}


impl ws::Handler for WsRouter {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<ws::Response> {
        let out = Rc::new(self.sender.clone());
        let req_resource = req.resource();
        let connect_uri = format!("{}{}{}", "ws://", WS_ADDRESS, req_resource.to_owned());
        let result = url::Url::parse(connect_uri.as_str());

        if let Ok(result) = result {
            let path = result.path();
            let query = result
                .query_pairs()
                .into_owned()
                .collect::<std::collections::HashMap<String, String>>();
            let token = query.get("token");
            let send_id = query.get("sendID");
            let platform = query.get("platform");

            if token == None || send_id == None || platform == None {
                return Ok(ws::Response::new(404, "Not Found", vec![]));
            }

            let token = token.unwrap();
            let send_id = send_id.unwrap();
            let platform = platform.unwrap().parse::<u8>().unwrap_or_default();

            let claims =
                jwt_auth::decode_token(token, AppState::get_config().jwt_secret.as_slice());

            let mut real_send_id = format!("");
            if claims.is_some() {
                let claims = claims.unwrap();
                let send_id = send_id.clone();
                if claims.user_id == send_id {
                    real_send_id = send_id
                }
            }

            match path {
                "/rust-im" => {
                    self.inner = Box::new(RustIMHandler {
                        ws: out.clone(),
                        send_id: real_send_id,
                        redis_client: self.redis_client.clone(),
                        platform,
                        ws_conn_map: self.ws_conn_map.clone(),
                    });
                    self.inner.on_request(req)
                }
                _ => Ok(ws::Response::new(404, "Not Found", vec![])),
            }
        } else {
            Ok(ws::Response::new(400, "Bad Request", vec![]))
        }
    }

    fn on_shutdown(&mut self) {
        self.inner.on_shutdown()
    }

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        self.inner.on_open(shake)
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        self.inner.on_message(msg)
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        self.inner.on_close(code, reason)
    }

    fn on_error(&mut self, err: ws::Error) {
        self.inner.on_error(err)
    }
}

pub fn launch() {
    let _ws_server = thread::spawn(|| {
        let redis_client = Rc::new(redis::Client::open(REDIS_ADDRESS).unwrap());

        let ws_conn_map: Rc<RefCell<HashMap<String, Vec<Rc<ws::Sender>>>>> =
            Rc::new(RefCell::new(HashMap::new()));

        ws::listen(WS_ADDRESS, |out| WsRouter {
            sender: out,
            inner: Box::new(NotFound),
            redis_client: redis_client.clone(),
            ws_conn_map: ws_conn_map.clone(),
        })
        .unwrap();
    });
}
