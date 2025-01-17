
fn main() {
    let bedrock_server_url: &str = "https://www.minecraft.net/en-us/download/server/bedrock";
    let body = reqwest::blocking::get(bedrock_server_url).unwrap().text().unwrap();
    println!("body = {body:?}");
}
