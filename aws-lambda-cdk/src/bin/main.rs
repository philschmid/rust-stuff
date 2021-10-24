use lambda_http::{
    handler as lambda_handler,
    lambda_runtime::{self, Error},
};
use lib::*;
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    lambda_runtime::run(lambda_handler(handler)).await?;
    Ok(())
}
