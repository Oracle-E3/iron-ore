use strum_macros::{EnumString};

#[derive(Debug, PartialEq, EnumString)]
enum ServerPlatform {
    Windows(String),
    Linux(String),
    WindowsPreview(String),
    LinuxPreview(String),
}

async fn get_bedrock_server_download_page(url: String) -> Result<String, reqwest::Error> {
    Ok(String::from("found"))
}