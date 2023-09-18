use anyhow::{anyhow, Result};
use eth2::{lighthouse_vc::http_client::ValidatorClientHttpClient, SensitiveUrl};

pub fn new_validator_client(
    validator_enpoint: &str,
    secret: String,
) -> Result<ValidatorClientHttpClient> {
    let url = SensitiveUrl::parse(validator_enpoint)
        .map_err(|_| anyhow::anyhow!("Parse eth2 endpoint error"))?;
    let client = ValidatorClientHttpClient::new(url, secret).map_err(|err| anyhow!("{err}"))?;
    Ok(client)
}
