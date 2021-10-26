use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://35.232.6.190/aggregation/metadata")
        .await?
        // .json::<HashMap<String, String>>()
        .text()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}