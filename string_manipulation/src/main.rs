use url::{ParseError, Position, Url};
#[derive(Debug)]
struct Repository {
    name: String,
    filter: Option<String>,
}

fn main() -> Result<(), ParseError> {
    // let hf_uri = String::from("hf://philschmid/infinity-sentiment//infinity/config.json");
    let hf_uri = String::from("hf://philschmid/infinity-sentiment");
    let parsed_hf_uri = Url::parse(hf_uri.as_str())?;
    let uri_without_schema: &str = &parsed_hf_uri[Position::BeforeHost..];

    let repository: Repository = match uri_without_schema.contains("//") {
        true => {
            let url_split: Vec<&str> = uri_without_schema.split("//").collect();
            Repository {
                name: String::from(url_split[0]),
                filter: Some(String::from(url_split[1])),
            }
        }
        _ => Repository {
            name: String::from(uri_without_schema),
            filter: None,
        },
    };

    Ok(())
}
