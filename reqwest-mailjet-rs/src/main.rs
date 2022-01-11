extern crate reqwest;
use std::env;

use reqwest::header;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mj_key = env::var("MJ_KEY").expect("$MJ_KEY is not set");
    let mj_secret = env::var("MJ_SECRET").expect("$MJ_SECRET is not set");

    let mut headers = header::HeaderMap::new();

    headers.insert("Content-Type", "application/json".parse().unwrap());

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.mailjet.com/v3.1/send")
        .basic_auth(mj_key, Some(mj_secret))
        .headers(headers)
        .body(r#"
{
		"Messages":[
				{
						"From": {
								"Email": "infinity@huggingface.co",
								"Name": "Hugging Face Team"
						},
						"To": [
								{
										"Email": "philipp@huggingface.co",
										"Name": "Philipp Schmid"
								}
						],
						"Subject": "Your email flight plan!",
						"TextPart": "Dear passenger 1, welcome to Mailjet! May the delivery force be with you!",
						"HTMLPart": "<h3>Dear passenger 1, welcome to <a href=\"https://www.mailjet.com/\">Mailjet</a>!</h3><br />May the delivery force be with you!"
				}
		]
	}
"#
        )
        .send()
        .await?;
    println!("{:?}", res);

    Ok(())
}
