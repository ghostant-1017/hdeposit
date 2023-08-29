use anyhow::{anyhow, Result};
use std::str::FromStr;
use url::Url;

pub async fn get_current_finality_block_number(base: &Url) -> Result<u64> {
    let endpoint = base.join("eth/v1/beacon/blocks/finalized")?;
    // data => body => execution_payload => block_number
    let response: serde_json::Value = reqwest::get(endpoint).await?.json().await?;
    let data = response
        .get("data")
        .ok_or(anyhow!("Missing field `data`"))?;
    let message = data
        .get("message")
        .ok_or(anyhow!("Missing field message"))?;
    let body = message.get("body").ok_or(anyhow!("Missing field body"))?;
    let execution_payload = body
        .get("execution_payload")
        .ok_or(anyhow!("Missing field `execution_payload`"))?;
    let block_number = execution_payload
        .get("block_number")
        .ok_or(anyhow!("Missing field `block_number`"))?;
    let block_number: String = serde_json::from_value(block_number.clone())?;
    let block_number = u64::from_str(&block_number)?;
    Ok(block_number)
}
