use axum::{routing::post, Router};
use pyo3::prelude::*;
use std::net::SocketAddr;

async fn test_route() -> &'static str {
    let fut = Python::with_gil(|py| {
        let asyncio = py.import("asyncio")?;

        // convert asyncio.sleep into a Rust Future
        pyo3_asyncio::tokio::into_future(asyncio.call_method1("sleep", (1.into_py(py),))?)
    })
    .unwrap();

    println!("sleeping for 1s");
    fut.await.unwrap();
    println!("done");
    "Response"
}

#[pyo3_asyncio::tokio::main]
async fn main() -> PyResult<()> {
    // get the pyo3_asyncio context in main
    let locals = Python::with_gil(pyo3_asyncio::tokio::get_current_locals)?;

    let app = Router::new().route(
        "/test",
        // scope your route handler with this pyo3_asyncio context
        post(move || pyo3_asyncio::tokio::scope(locals.clone(), test_route())),
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
