use actix::{Actor, StreamHandler};
use actix_web::{HttpRequest, web, HttpResponse, Error};
use actix_web_actors::ws;

pub struct HellWs;


impl Actor for HellWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for HellWs {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Ping(msg))   => { ctx.pong(&msg); }
            Ok(ws::Message::Text(text))  => { ctx.text(text); }
            Ok(ws::Message::Binary(bin)) => { ctx.binary(bin); }
            Ok(_) => { }
            Err(e) => { panic!("failed to handle ws message: {}", e); }
        }
    }
}

impl HellWs {
    pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
        let ws = HellWs { };
        let resp = ws::start(ws, &req, stream);
        println!("HellWs::index: '{:?}'", resp);
        resp
    }
}
