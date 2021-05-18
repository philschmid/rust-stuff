use actix_web::{error, get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

mod calculator;
use calculator::Calculator;

#[derive(Serialize)]
struct HealthStatus {
    status: String,
}

#[derive(Serialize)]
struct CalculatorResult {
    result: i32,
}

#[derive(Deserialize)]
struct CalculatorInput {
    #[serde(default)] // default = 0
    number1: i32,
    #[serde(default)] // default = 0
    number2: i32,
}

// #[derive(Deserialize)]
// struct CalculatorMultiplicationInput {
//     number1: Option<i32>,
//     number2: Option<i32>,
// }

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().header("x-time", "2").json(HealthStatus {
        status: String::from("healthy"),
    })
}

#[post("/add")]
async fn add(payload: web::Json<CalculatorInput>) -> Result<HttpResponse, Error> {
    println!("{}", payload.number1);
    let result = Calculator::add(payload.number1, payload.number2);
    Ok(HttpResponse::Ok().json(CalculatorResult { result: result }))
}

#[post("/div")]
async fn div(payload: web::Json<CalculatorInput>) -> impl Responder {
    println!("{}", payload.number1);
    let result = Calculator::div(payload.number1, payload.number2);

    match result {
        Ok(result) => HttpResponse::Ok().json(CalculatorResult {
            result: result as i32,
        }),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(
                // Json extractor configuration for this resource.
                web::JsonConfig::default().error_handler(|err, _req| {
                    error::InternalError::from_response(
                        "",
                        HttpResponse::BadRequest()
                            .content_type("application/json")
                            .body(format!(r#"{{"error":"{}"}}"#, err)),
                    )
                    .into()
                }),
            )
            .service(health)
            .service(add)
            .service(div)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
