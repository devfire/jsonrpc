mod objects;

use actix_web::{App, HttpServer, Responder, web};
use objects::JsonRpcRequest;

async fn handle_json(data: web::Json<JsonRpcRequest>) -> impl Responder {
    println!("Received JSON: {:?}", data);
    "Data received"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::post().to(handle_json)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
