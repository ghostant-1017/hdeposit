use anyhow::{Result, anyhow};
use url::Url;

pub async fn get_current_finality(base: &Url) -> Result<u64> {
    let endpoint = base.join("eth/v1/beacon/states/finalized/finality_checkpoints")?;
    let response: serde_json::Value = reqwest::get(endpoint).await?.json().await?;
    let data = response.get("data").ok_or(anyhow!("Missing field `data`"))?;
    let finalized = data.get("finalized").ok_or(anyhow!("Missing field `finalized`"))?;
    let epoch = finalized.get("epoch").ok_or(anyhow!("Missing field `epoch`"))?;
    let epoch = serde_json::from_value(epoch.clone())?;
    Ok(epoch)
}