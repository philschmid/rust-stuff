use lambda_http::{
    lambda_runtime::{Context, Error},
    IntoResponse, Request, RequestExt,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Default)]
struct Human {
    #[serde(default)]
    name: String,
    #[serde(default)]
    age: u8,
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct ResponseBody {
    message: String,
    born: u16,
}

pub async fn handler(request: Request, _: Context) -> Result<impl IntoResponse, Error> {
    log::info!("{:?}", request);
    println!("{:?}", request);

    // deserialize body
    let body: Human = match request.payload() {
        Ok(body) => body.unwrap(),
        Err(err) => {
            log::error!("{:?}", err);
            return Ok(json!({"error":&err.to_string()}).into_response());
        }
    };
    // calculate born year
    let born: u16 = 2021 - (body.age as u16);

    // create Response struct
    let res = ResponseBody {
        message: format!("{name} was born in {born}", name = body.name, born = born),
        born: born,
    };
    log::info!("{:?}", res);

    // respond
    Ok(serde_json::to_string(&res).unwrap().into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::http::{self, HeaderValue};

    #[tokio::test]
    async fn test_lambda_handler() {
        let data = r#"{
            "name":"Philipp",
            "age":28
        }"#;
        let mut request = Request::new(lambda_http::Body::from(data));
        request.headers_mut().insert(
            http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        let response = handler(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();

        assert_eq!(response.status(), 200)
    }
}
