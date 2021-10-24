use std::collections::HashMap;

use lambda_runtime::{Context, Error};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Default)]
struct Human {
    #[serde(default)]
    name: String,
    #[serde(default)]
    age: u8,
}

#[derive(Debug, Deserialize)]
pub struct ProxyRequest {
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Serialize)]
pub struct ProxyResponse {
    #[serde(rename = "statusCode")]
    status_code: u16,

    body: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct ResponseBody {
    message: String,
    born: u16,
}

pub async fn handler(request: ProxyRequest, _: Context) -> Result<ProxyResponse, Error> {
    log::info!("{:?}", request);
    let body: Human = match serde_json::from_str(request.body.as_str()) {
        Ok(val) => val,
        Err(err) => {
            log::info!("{:?}", err);
            return Ok(ProxyResponse {
                status_code: 403,
                body: json!({"error":err.to_string()}).to_string(),
            });
        }
    };

    log::info!("{:?}", body);
    // calculate born year
    let born: u16 = 2021 - (body.age as u16);

    // create Response struct
    let res = ResponseBody {
        message: format!("{name} was born in {born}", name = body.name, born = born),
        born: born,
    };
    log::info!("{:?}", res);

    // respond
    Ok(ProxyResponse {
        status_code: 200,
        body: serde_json::to_string(&res).unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lambda_handler() {
        let data = r#"{
            "name":"Philipp",
            "age":28
        }"#;
        let request = ProxyRequest {
            headers: HashMap::new(),
            body: data.to_string(),
        };
        let response = handler(request, Context::default())
            .await
            .expect("expected Ok(_) value");
        assert_eq!(response.status_code, 200)
    }
}
