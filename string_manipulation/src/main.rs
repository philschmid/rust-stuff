use url::{ParseError, Position, Url};

fn main() -> Result<(), ParseError> {
    let hf_uri = String::from("hf://philschmid/infinity-sentiment//infinity/config.json");
    let parsed_hf_uri = Url::parse(hf_uri.as_str())?;
    let uri_without_schema: &str = &parsed_hf_uri[Position::BeforeHost..];

    let filter = match uri_without_schema.contains("//") {
        true => {
            let url_split: Vec<&str> = uri_without_schema.split("//").collect();
            url_split[1]
        }
        _ => "",
    };
    println!("filter: {}", filter);
    println!("uri_without_schema: {}", uri_without_schema);

    Ok(())
}
