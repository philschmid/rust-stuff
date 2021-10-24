use lambda_http::{
    lambda_runtime::{Context, Error},
    IntoResponse, Request, RequestExt, Response,
};
use serde::{Deserialize, Serialize};

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

    // deserialize body
    let body: Human = match request.payload() {
        Ok(body) => body.unwrap(),
        Err(err) => Err(err)?,
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
    use serde_json::json;

    #[tokio::test]
    async fn test_lambda_handler() {
        let data = r#"{
            "name":"Philipp",
            "age":28
        }"#;
        let request = Request::new(lambda_http::Body::from(data));
        let data = json!(
        {
            "name":"Philipp",
            "age":28
        });
        let expected = data.into_response();
        let response = handler(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
