// use anyhow::Result;
use pyo3::prelude::*;

type PyFunction = Py<PyAny>;
use tower::ServiceBuilder;

use axum::{
    extract::Extension, http::Method, response::IntoResponse, routing::post, AddExtensionLayer,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use std::net::SocketAddr;

#[derive(Deserialize, Debug)]
pub struct PredictRequest {
    inputs: String,
}

#[derive(FromPyObject, Debug, Serialize)]
struct TextClassificationResponse {
    #[pyo3(item)]
    label: String,
    #[pyo3(item)]
    score: f32,
}

async fn predict_route(payload: PredictRequest, handler: Py<PyAny>) -> impl IntoResponse {
    println!("{:?}", handler);
    let fut = Python::with_gil(|py| {
        let coro = handler.call1(py, (payload.inputs.as_str(),)).unwrap();
        pyo3_asyncio::tokio::into_future(coro.as_ref(py))
    })
    .unwrap();
    println!("no infrence yet");
    let res = fut.await.unwrap();

    let res = Python::with_gil(|py| -> PyResult<Vec<TextClassificationResponse>> {
        res.extract::<Vec<TextClassificationResponse>>(py)
    });
    Json(res.unwrap())
}

// async fn predict_route(
//     Json(payload): Json<PredictRequest>,
//     Extension(handler): Extension<Py<PyAny>>,
// ) -> impl IntoResponse {
async fn handle(input: &str, handler: &PyFunction) -> Vec<TextClassificationResponse> {
    let fut = Python::with_gil(|py| {
        let coro = handler.call1(py, (input,)).unwrap();
        pyo3_asyncio::tokio::into_future(coro.as_ref(py))
    })
    .unwrap();
    println!("no infrence yet");
    println!("{:?}", handler);
    let res = fut.await.unwrap();

    let res = Python::with_gil(|py| -> PyResult<Vec<TextClassificationResponse>> {
        res.extract::<Vec<TextClassificationResponse>>(py)
    });
    res.unwrap()
}

#[pyo3_asyncio::tokio::main]
async fn main() -> PyResult<()> {
    // get the pyo3_asyncio context in main
    let pyo3_asyncio_event_loop = Python::with_gil(pyo3_asyncio::tokio::get_current_locals)?;

    const PYTHON_MODULE: &str = include_str!("../handler.py");

    let pipeline: PyFunction = Python::with_gil(|py| {
        let pipeline = PyModule::from_code(py, PYTHON_MODULE, "handler.py", "handler").unwrap();
        let handler = pipeline.getattr("handle").unwrap();
        handler.extract().unwrap()
    });
    println!("model loaded");

    let res = handle("I like you. I like you.I like you.I like you.I like you.I like you.I like you.I like you.I like you.I like you.I like you.I like you.I like you.I like you.I like you.I like you.I like you.", &pipeline);
    println!("do i come before infernece?");
    println!("{:?}", res.await);

    let middleware_stack = ServiceBuilder::new().layer(AddExtensionLayer::new(pipeline));

    let app = Router::new()
        .route(
            "/predict",
            post(
                move |Json(payload): Json<PredictRequest>,
                      Extension(handler): Extension<Py<PyAny>>| {
                    pyo3_asyncio::tokio::scope(
                        pyo3_asyncio_event_loop.clone(),
                        predict_route(payload, handler),
                    )
                },
            ),
        )
        .layer(middleware_stack);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

// #[pyo3_asyncio::tokio::main]
// async fn main() -> PyResult<()> {
//     const PYTHON_MODULE: &'static str = include_str!("../handler.py");

//     let fut = Python::with_gil(|py| {
//         let pipeline_file =
//             PyModule::from_code(py, PYTHON_MODULE, "handler.py", "handler").unwrap();
//         println!("{:?}", pipeline_file.hasattr("clx").unwrap());
//         let handler = pipeline_file.getattr("handle").unwrap();
//         let coro = handler.call1(("hello i love you",));
//         pyo3_asyncio::tokio::into_future(coro?)
//     })?;

//     println!("sleeping for 1s");
//     let res = fut.await?;
//     println!("res: {:?}", res);
//     let res = Python::with_gil(|py| -> PyResult<Vec<TextClassificationResponse>> {
//         Ok(res.extract::<Vec<TextClassificationResponse>>(py).unwrap())
//     });
//     println!("res: {:?}", res);
//     println!("done");

//     Ok(())
// }
