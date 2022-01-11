use std::{
    collections::HashMap,
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Claims {
    sub: String, // Subject (unique id for process)
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: u64,   // Issued at (as UTC timestamp)
    iss: String, // Issuer
    aud: String, // audience
}

impl Claims {
    fn new(aud: &str) -> Claims {
        Claims {
            sub: "infinity".to_string(),
            exp: 1000000000000,
            iat: default_iat(),
            aud: aud.to_string(),
            iss: "huggingface".to_string(),
        }
    }
}

fn default_iat() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn create_jwt(claims: Claims, key: &[u8]) -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(key)?,
    )
}

fn validate_jwt_with_jwk<T>(
    token: &str,
    jwk: &serde_json::Value,
) -> Result<TokenData<T>, jsonwebtoken::errors::Error>
where
    T: serde::de::DeserializeOwned,
{
    let validation = Validation {
        iss: Some("huggingface".to_string()),
        algorithms: vec![Algorithm::RS256],
        ..Validation::default()
    };
    decode::<T>(
        token,
        &DecodingKey::from_rsa_components(jwk["n"].as_str().unwrap(), jwk["e"].as_str().unwrap()),
        &validation,
    )
}
// openssl genrsa -out private-key.pem 2048
// openssl rsa -in private-key.pem -pubout -out public-key.pem

fn main() {
    let prv_key = include_bytes!("../prv_key.pem");
    let my_claims = Claims::new("prophia");

    let jwt = create_jwt(my_claims, prv_key).unwrap();
    println!("{}", jwt);

    // decode jwt
    let jwk_file = fs::File::open("jwk.json").expect("file should open read only");
    let jwk: serde_json::Value =
        serde_json::from_reader(jwk_file).expect("file should be proper JSON");
    let res = validate_jwt_with_jwk::<Claims>(&jwt, &jwk["keys"][0]);
    println!("{:?}", res);
}
