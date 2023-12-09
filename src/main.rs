use std::net::IpAddr;
use serde::Deserialize;

mod error;
use error::*;

#[tokio::main]
async fn main() -> Result<(), LumenError> {
    let bridges = get_bridges().await?;
    println!("{:?}", bridges);

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Bridge {
    id: String,

    #[serde(rename = "internalipaddress")]
    ip: IpAddr,
    port: u16,
}

async fn get_bridges() -> Result<Vec<Bridge>, LumenError> {
    let res = reqwest::get("http://discovery.meethue.com")
        .await?
        .bytes()
        .await?;

    Ok(serde_json::from_slice(&res)?)
}
