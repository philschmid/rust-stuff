use lambda_runtime::{handler_fn, Error};

use lib::*;
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Here i could do other things

    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    lambda_runtime::run(handler_fn(handler)).await?;
    Ok(())
}
