use actix_web::{ web, App, HttpResponse, HttpServer};
use actix_web::web::Data;

use pyo3::prelude::*;
use serde::Deserialize;


#[derive(Deserialize)]
struct RequestData {
    inputs: String,
}

/// deserialize `Info` from request's body, max payload size is 4kb
async fn index(
    text_classification: Data<Py<PyAny>>,
    r: web::Json<RequestData>,
) -> HttpResponse {
    println!("{}", r.inputs);
    // println!("{:?}",counter2);

    // let pred = text_classification.call1(py,("i love you.",)).unwrap();
    println!("{:?}",text_classification);

    HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"test\":\"success\"}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let gil = Python::acquire_gil();
    let transformers = PyModule::import(gil.python(), "transformers").unwrap();
    let text_classification: Data<Py<PyAny>> =  Data::new(transformers
        .getattr("pipeline")
        .unwrap()
        .call1(("text-classification",))
        .unwrap()
        .extract()
        .unwrap());

    
        HttpServer::new(move || {

            App::new()
                // .data(text_classification.clone()) // add thread-local state
                .service(
                    web::resource("/")
                        // change json extractor configuration
                        .route(web::post().to(index)),
                )
            })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    }
