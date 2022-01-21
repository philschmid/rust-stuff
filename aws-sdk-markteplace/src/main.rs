use std::env;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_marketplacemetering::{Client, Error};
use aws_types::config::Config;

async fn get_credentials() -> Config {
    // local test
    env::set_var("AWS_PROFILE", "hf-sm");
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    shared_config
}

const PRODUCT_CODE: &str = "x";
// const NONCE: &str = "hf-sm";
const PUBLIC_KEY_VERSION: i32 = 1;

async fn register_usage() -> Result<(), Error> {
    let shared_config = get_credentials().await;
    let client = Client::new(&shared_config);
    let req = client
        .register_usage()
        .product_code(PRODUCT_CODE.to_string()) // Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.
        .public_key_version(PUBLIC_KEY_VERSION); // Public Key Version provided by AWS Marketplace
                                                 // .nonce(NONCE); // (Optional) To scope down the registration to a specific running software instance and guard against replay attacks.
    req.send().await.unwrap();
    Ok(())
}

#[tokio::main]
async fn main() {
    register_usage().await.unwrap();
}
