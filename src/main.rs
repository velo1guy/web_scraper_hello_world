use std::process;
//use mini_redis::{client, Result};
use anyhow::Result; 

#[tokio::main]
async fn main() -> Result<()> {

    env_logger::init();
    let resp = reqwest::get("https://google.com")
        // .await?
        // .json::<HashMap<String, String>>()
        .await?;

    println!("{:?}", resp.text().await);

    log::info!("hello, world!");

    Ok(())
}

