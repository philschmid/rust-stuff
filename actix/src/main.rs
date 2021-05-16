use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize};


#[derive(Serialize)]
struct HealthStatus {
    status: String,
}

#[get("/health")]
async fn health() -> impl Responder  {
    HttpResponse::Ok().json(HealthStatus {
        status: String::from("healthy"),
    })

}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}