use std::env;

use dotenv::dotenv;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = env::var("SLACK_WEBHOOK_URL")?;
    let json_request = format!(r#"{{"text": "ジョブが終了しました！"}}"#);
    let client = Client::new();
    let res = client.post(url).body(json_request).send().await?;
    let res = res.text().await?;
    println!("{}", res);
    Ok(())
}
