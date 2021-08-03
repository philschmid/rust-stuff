use actix_web::{
    error, get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};

mod calculator;
use calculator::Calculator;

#[derive(Serialize, Deserialize)]
struct HealthStatus {
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CalculatorResult {
    result: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct CalculatorInput {
    // #[serde(default)] // default = 0
    number1: i32,
    // #[serde(default)] // default = 0
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

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType()
            .content_type("application/json")
            .body(format!(r#"{{"error":"{}"}}"#, detail)),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity()
                .content_type("application/json")
                .body(format!(r#"{{"error":"{}"}}"#, detail))
        }
        _ => HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error":"{}"}}"#, detail)),
    };
    error::InternalError::from_response(err, resp).into()
}

use actix_web::Either;

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

async fn index() -> RegisterResult {
    let task = "test2";
    match task {
        "test" => Either::A(HttpResponse::BadRequest().body("Bad data")),
        _ => Either::B(Ok("Hello!")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .app_data(
                web::JsonConfig::default()
                    // register error_handler for JSON extractors.
                    .error_handler(json_error_handler),
            )
            .service(health)
            .service(add)
            .service(div)
            .route("/x", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App, Error};

    #[actix_rt::test]
    async fn test_health() -> Result<(), Error> {
        let app = App::new().service(health);
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/health").to_request();

        let resp: HealthStatus = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.status, "healthy");
        Ok(())
    }
    #[actix_rt::test]
    async fn test_add() -> Result<(), Error> {
        let app = App::new().service(add);
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::post()
            .uri("/add")
            .set_json(&CalculatorInput {
                number1: 2,
                number2: 43,
            })
            .to_request();

        let resp: CalculatorResult = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.result, 2 + 43);
        Ok(())
    }
}
