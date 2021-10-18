use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error};
use aws_sdk_s3;

use aws_types::config::Config;
use std::{env, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::fs::File;

const REGION: &str = "us-east-1";
const AWS_PROFILE: &str = "hf-sm";
const S3_BUCKET: &str = "comprehend-autonlp-bucket";

async fn get_credentials() -> Config {
    // TODO: adjust for AWS Lambda to work
    match env::var("AWS_ACCESS_KEY_ID") {
        Ok(_) => (),
        Err(_) => env::set_var("AWS_PROFILE", AWS_PROFILE),
    }
    let region_provider = RegionProviderChain::default_provider().or_else(REGION);
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    shared_config
}

async fn list_tables() -> Vec<String> {
    let shared_config = get_credentials().await;
    let client = Client::new(&shared_config);
    let req = client.list_tables().limit(10);
    let table_names = req.send().await.unwrap().table_names.unwrap();
    table_names
}

async fn upload_local_file_to_s3() {
    let file_name = "data.json";

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    let v: Value = serde_json::from_str(data).unwrap();
    serde_json::to_writer(&File::create(file_name).unwrap(), &v).unwrap();

    // aws credentials and client
    let shared_config = get_credentials().await;
    let client = aws_sdk_s3::Client::new(&shared_config);

    // read json file and define key
    let body = aws_sdk_s3::ByteStream::from_path(Path::new(file_name))
        .await
        .unwrap();

    client
        .put_object()
        .bucket(S3_BUCKET)
        .key(file_name)
        .body(body)
        .send()
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}
async fn upload_struct_to_s3() {
    let p = Person {
        name: "Philipp Schmid".to_string(),
        age: 25,
        phones: vec!["+123".to_string(), "+456".to_string()],
    };

    // aws credentials and client
    let shared_config = get_credentials().await;
    let client = aws_sdk_s3::Client::new(&shared_config);

    // convert struct to Vec<[u8]>
    let body = aws_sdk_s3::ByteStream::from(serde_json::to_vec(&p).unwrap());

    client
        .put_object()
        .bucket(S3_BUCKET)
        .key("struct.json")
        .body(body)
        .send()
        .await
        .unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // // list dynamoDB tables
    // let table_names = list_tables().await;
    // println!("Current DynamoDB tables: {:?}", table_names);

    // Uploads local file to s3
    // upload_local_file_to_s3().await;

    // Uploads struct as json to s3
    upload_struct_to_s3().await;

    Ok(())
}
