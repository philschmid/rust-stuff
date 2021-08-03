use error_chain::error_chain;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Deserialize, Debug)]
struct Sibling {
    rfilename: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Repository {
    model_id: String,
    private: bool,
    sha: String,
    last_modified: String,
    tags: Vec<String>,
    siblings: Vec<Sibling>,
    #[serde(rename = "library_name")]
    library_name: String,
    config: serde_json::Value,
}

#[tokio::main]
async fn main() -> Result<()> {
    let base_url = "https://huggingface.co";
    let repo = "philschmid/infinity-sentiment";

    let request_url = format!("https://huggingface.co/api/models/{repo}", repo = repo);
    let response = reqwest::get(request_url).await?;
    let repository_information: Repository = response.json().await?;

    for file in repository_information.siblings {
        let remote_file_url = format!(
            "{base_url}/{repo}/resolve/main/{file_path}",
            base_url = base_url,
            repo = repo,
            file_path = file.rfilename,
        );
        load_file(remote_file_url, file.rfilename).await?
    }
    Ok(())
}

async fn load_file(file_url: String, file_path: String) -> Result<()> {
    let prefix = "infinity";
    let path = Path::new(prefix).join(file_path);
    let directory = path.parent().unwrap();

    let requested_file = reqwest::get(file_url).await?;

    if !directory.exists() {
        fs::create_dir(directory)?;
    }

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}, {:?}", why, path),
        Ok(file) => file,
    };
    let content = requested_file.text().await?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
