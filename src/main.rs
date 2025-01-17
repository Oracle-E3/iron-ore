use regex::Regex;

fn main() {
    let bedrock_server_url: &str = "https://www.minecraft.net/en-us/download/server/bedrock";
    let body = reqwest::blocking::get(bedrock_server_url).unwrap().text().unwrap();
    // println!("body = {body:?}");
    let latest_download_re = Regex::new(r"https?://[a-zA-Z0-9\-\./]+/bin-(?<os>\w+)/bedrock-server-(?<version>[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+).zip").unwrap();
    let Some(results) = latest_download_re.captures(body.as_str()) else {
        println!("Can't find download link!");
        return;
    };
    println!("OS: {} - Version {}", &results["os"], &results["version"]);
}
