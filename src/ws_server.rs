use std::{thread, vec};
use url;
use ws;

use crate::config::AppState;
use crate::jwt_auth;

struct WsRouter {
    sender: ws::Sender,
    inner: Box<dyn ws::Handler>,
}

struct RustIMHandler {
    ws: ws::Sender,
    send_id: String,
}

impl ws::Handler for RustIMHandler {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("{} on_message {:#?}", self.send_id, msg);
        self.ws.send(format!("reply to {}, pong!", self.send_id))
    }
}

struct NotFound;
impl ws::Handler for NotFound {}

const WS_ADDR: &'static str = "127.0.0.1:54321";

impl ws::Handler for WsRouter {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<ws::Response> {
        let out = self.sender.clone();
        let req_resource = req.resource();
        let connect_uri = format!("{}{}{}", "ws://", WS_ADDR, req_resource.to_owned());
        let result = url::Url::parse(connect_uri.as_str());

        if let Ok(result) = result {
            let path = result.path();
            let query = result
                .query_pairs()
                .into_owned()
                .collect::<std::collections::HashMap<String, String>>();
            let token = query.get("token");
            let send_id = query.get("sendID");

            if token == None || send_id == None {
                return Ok(ws::Response::new(404, "Not Found", vec![]));
            }

            let token = token.unwrap();
            let send_id = send_id.unwrap();

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
                        ws: out,
                        send_id: real_send_id,
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
        ws::listen(WS_ADDR, |out| WsRouter {
            sender: out,
            inner: Box::new(NotFound),
        })
        .unwrap();
    });
}
