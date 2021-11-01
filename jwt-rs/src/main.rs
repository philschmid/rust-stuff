use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
extern crate num_cpus;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Claims {
    sub: String,         // Subject (unique id for process)
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: u64,   // Issued at (as UTC timestamp)
    iss: String, // Issuer
    n_cpus: u8, // number of cpus
    n_gpus: u8, // number of gpus
    customer_id: String, // customerID
}

impl Claims {
    fn new(customer_id: &str) -> Claims {
        Claims {
            sub: default_sub(),
            exp: 1000000000000,
            iat: default_iat(),
            iss: default_iss(),
            n_cpus: default_n_cpus(),
            n_gpus: default_n_gpus(),
            customer_id: customer_id.to_string(),
        }
    }
}

fn default_sub() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn default_iss() -> String {
    "infinity".to_string()
}

fn default_n_cpus() -> u8 {
    // count logical cores
    num_cpus::get() as u8
    // count physical cores
    // num_cpus::get_physical();
}

fn default_iat() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn default_n_gpus() -> u8 {
    0
}

fn issue_token(claims: Claims, key: &str) -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_bytes()),
    )
}

fn validate_token<T>(token: &str, key: &str) -> Result<TokenData<T>, jsonwebtoken::errors::Error>
where
    T: serde::de::DeserializeOwned,
{
    let validation = Validation {
        iss: Some("infinity".to_string()),
        ..Validation::default()
    };
    decode::<T>(
        token,
        &DecodingKey::from_secret(key.as_bytes()),
        &validation,
    )
}

const CUSTOMER_ID: &str = "alphasense_1";

fn main() {
    let key = "secret";
    let my_claims = Claims::new(CUSTOMER_ID);
    let token = match issue_token(my_claims, &key) {
        Ok(t) => t,
        Err(err) => panic!("{}", err), // in practice you would return the error
    };
    // let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjEwMDAwMDAwMDAwMDAsImlhdCI6MTYzNTYwOTExMiwiaXNzIjoiaW5maW5pdHkiLCJuQ3B1cyI6OCwibkdwdXMiOjEsImN1c3RvbWVySWQiOiJhbHBoYXNlbnNlXzEifQ.Gi6DvmbWSWYVIOl0YRpc-lFL4J6BsNs-F4cV4DXrRCE";
    println!("{}", token);
    let token_data = match validate_token::<Claims>(&token, key) {
        Ok(c) => c.claims,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
            ErrorKind::InvalidSignature => panic!("JWT token malformed"), // Example on how to handle a specific error
            _ => panic!("{}", err),
        },
    };

    println!("{:?}", token_data);
    // println!("{:?}", token_data.header);
}
