use lambda_http::{Body, Context, IntoResponse, Request, handler, lambda_runtime};
use serde_json::{ json};
use serde::{Serialize, Deserialize};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(hello)).await?;
    Ok(())
}

#[derive(Serialize, Deserialize,Debug,)]
pub struct User {
  pub name: String,
  pub age: u16,
  pub phones: Vec<String>,
}

async fn hello(r: Request, _: Context) -> Result<impl IntoResponse, Error> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    println!("{:?}",r);
    let user:User = match r.body() {
        Body::Text(data) => serde_json::from_str(data)?,
        _ => panic!("error")
    };
    
    println!("{:?}",user);
    // let event: User = serde_json::from_value(r.body())?;
    // let v: Value = serde_json::from_str()?;

    Ok(json!(format!("Hello {}",user.name)))
}

#[cfg(test)]
mod tests {
    use lambda_http::Body;

    use super::*;

    // #[tokio::test]
    // async fn hello_handles() {
    //     let request = Request::default();
    //     let expected = json!({
    //         "message": "Go Serverless v1.0! Your function executed successfully!"
    //     })
    //     .into_response();
    //     let response = hello(request, Context::default())
    //         .await
    //         .expect("expected Ok(_) value")
    //         .into_response();
    //     assert_eq!(response.body(), expected.body())
    // }
    #[tokio::test]
    async fn hello_handles_body() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let body:Body = lambda_http::Body::from(data);
        let request = Request::new(body);
        let expected = json!(
           "Hello John Doe"
        )
        .into_response();
        let response = hello(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    
}
}
