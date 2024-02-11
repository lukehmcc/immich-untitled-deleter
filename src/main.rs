use env_file_reader::read_file;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use regex::bytes::Regex;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Some basic definitions
    let album_endpoint = String::from("/api/album");

    // First get env vars
    let env_variables = read_file("example/.env")?;
    let server_addr = String::from(&env_variables["SERVER_ADDRESS"]);
    let api_key = String::from(&env_variables["API_KEY"]);
    println!("{:?}", api_key);
    println!("{:?}", server_addr);

    // Now get albums
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-api-key"),
        HeaderValue::from_str(&api_key)?,
    );
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}{}", server_addr, album_endpoint))
        .headers(headers.clone())
        .send()
        .await?;

    // Handle the response as needed
    println!("Status: {}", response.status());
    let json_list: Value = serde_json::from_str(&response.text().await?).unwrap();

    for i in 0..json_list.as_array().unwrap().len() {
        // Regex the albums to find the bad ones
        let re = Regex::new(r"Untitled\((?:\d+)\)").unwrap();
        if re.is_match(json_list[i]["albumName"].to_string().as_bytes()) {
            let id_quoted: String = json_list[i]["id"].to_string();
            let id: &str = &id_quoted[1..id_quoted.len() - 1];
            let album_url: String = format!("{}{}{}{}", server_addr, album_endpoint, "/", id);
            println!("Deleting: {:?}", album_url);
            let response = client.delete(album_url)
                .headers(headers.clone())
                .send()
                .await?;

            if response.status().is_success() {
                println!("DELETE {:?} successful", id);
            } else {
                println!("DELETE {:?} failed: {}", id, response.status());
            }
        }
    }

    // we're in the clear!
    println!("Have a nice one (:");

    Ok(())
}