use reqwest::header::AUTHORIZATION;
use reqwest::Client;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub fullname: String,
    pub email: String,
    pub email_verified: bool,
    pub plan: String,
    pub period_end: Option<String>,
    pub avatar_url: String,
    pub orgs: Vec<Org>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Org {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub fullname: String,
    pub email: Option<String>,
    pub api_token: String,
    pub period_end: Option<String>,
    pub plan: String,
    pub avatar_url: String,
    pub role_in_org: String,
}

const AUTH_URL: &str = "https://huggingface.co/api/whoami-v2";
const TOKEN: &str = "hf_x";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let resp = client
        .get(AUTH_URL)
        .header(AUTHORIZATION, format!("Bearer {}", TOKEN))
        .send()
        .await?
        .json::<User>()
        .await?;

    println!("{:#?}", resp);
    Ok(())
}
