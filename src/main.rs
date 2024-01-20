use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
struct ChatMessage {
    message: String,
}

async fn chatbot(message: web::Json<ChatMessage>) -> impl Responder {
    let response = api::get_response(&message.message).await.unwrap();
    HttpResponse::Ok().json(ChatMessage { message: response })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/chatbot").route(web::post().to(chatbot)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
