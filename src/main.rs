mod persistante;

use std::env;
use env_logger;
use actix::{Actor, Addr};
use guitite::{Client, Server};
use actix_web_actors::ws;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, get, web};

use crate::persistante::Persistance;

struct Conection {
    pub server: Addr<Server<Persistance>>,
    pub token: String,
}

#[get("/ws/{file_name}")]
async fn client(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Conection>,
    path: web::Path<String>
) -> Result<HttpResponse, Error> {
    
    let conection = srv.get_ref();
    let expected = format!("Auth-{}", conection.token);
    
    let is_valid = req.headers()
            .get("sec-websocket-protocol")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.split(',').next())
            .map(|first| first.unwrap().trim() == expected)
            .unwrap_or(false);
    
    if !is_valid {
        log::warn!("Intento de conexión no autorizado");
        log::warn!("{expected}");
        log::debug!("protocol: {:#?}", req.headers().get("sec-websocket-protocol"));
        return Ok(HttpResponse::Unauthorized().finish());
    }
    
    ws::start( Client::new(path.as_str(), conection.server.clone()), &req, stream, )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::debug!("Logs set to 'debug'");
    
    let username = match env::var("USER_NAME") {
        Ok(s) => s,
        Err(_) => {
            log::warn!("YOU HAVE TO CONFIGURE THE ENV VARIABLE: USER_NAME");
            log::warn!("USING DEFAULT: ADMIN");
            "ADMIN".to_string()
        },
    };
    
    let password = match env::var("USER_PASW") {
        Ok(s) => s,
        Err(_) => {
            log::warn!("YOU HAVE TO CONFIGURE THE ENV VARIABLE: USER_PASW");
            log::warn!("USING DEFAULT: ADMIN");
            "ADMIN".to_string()
        },
    };

    let port: u16 = match env::var("PORT") {
        Ok(s) => s.parse().unwrap_or(3030),
        Err(_) => 3030,
    };
    

    let server = Server::new_with_actor(Persistance::new).start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Conection { server: server.clone(), token: format!("{}-{}", username, password) }))
            .service(client)
    })
    .workers(2)
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
