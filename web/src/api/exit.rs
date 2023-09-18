use anyhow::anyhow;
use eth2::{
    types::{Keypair, SignedVoluntaryExit, ValidatorStatus, VoluntaryExit},
    BeaconNodeHttpClient,
};
use ethers::types::{Address, Signature, Sign};
use serde::de;
use storage::models::{
    insert_exit_message, query_keystore_by_public_key, select_validator_by_index,
};
use std::str::FromStr;

use crate::utils::get_current_epoch;

use super::*;

struct ExitMessage(u64);

impl ToString for ExitMessage {
    fn to_string(&self) -> String {
        format!("I WANT TO EXIT VALIDATOR INDEX: {}", self.0)
    }
}

#[derive(Debug)]
pub struct Params {
    validator_index: u64,
    signature: Signature,
}
impl<'a> Deserialize<'a> for Params {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a> 
    {
        let mut m_result = serde_json::Value::deserialize(deserializer)?;

        let validator_index: u64 = serde_json::from_value(m_result["validator_index"].take()).map_err(de::Error::custom)?;
        let signature: Signature = Signature::from_str(&m_result["signature"].take().as_str().ok_or(de::Error::custom("signature missing"))?).map_err(de::Error::custom)?;
        Ok(Self { validator_index, signature })
    }
}

#[derive(Debug, Serialize)]
pub struct Response;

pub async fn post_exit(
    State(server): State<Server>,
    Json(params): Json<Params>,
) -> Result<Json<Response>, AppError> {
    let db = server.pool.get().await?;

    // 1. Select the validator index related `HellmanValidator`
    let validator_index = params.validator_index;
    let validator = select_validator_by_index(&db, validator_index)
        .await?
        .ok_or(anyhow!("Validator not found"))?;
    let validator_status = validator
        .data
        .ok_or(anyhow!("Validator status unknown"))?
        .status
        .superstatus();
    if validator_status != ValidatorStatus::Active {
        return Err(anyhow!("Validator is not active, status: {}", validator_status).into());
    }

    // 2. *****Verify the validator is belong to the user*****
    let (_prefix, address) = validator.withdrawal_credentials.0.split_at(12);
    let address = Address::from_slice(address);
    let exit_message = ExitMessage(validator_index).to_string();
    params
        .signature
        .verify(exit_message.clone(), address)
        .map_err(|_| anyhow!("Signature verify error"))?;

    // 3. Prepare to generate `VoluntaryExit`
    let pubkey = validator.pubkey.as_hex_string();
    let (_, pubkey) = pubkey.split_at(2);
    let keystore = query_keystore_by_public_key(&db, pubkey)
        .await?
        .ok_or(anyhow!("Keystore not found"))?;
    let keypair = keystore
        .keystore
        .decrypt_keypair(server.password.as_bytes())
        .map_err(|_| anyhow!("KeyPass error"))?;

    // 4. Insert into db and broadcast
    let epoch = get_current_epoch(&server.clock)?;
    let voluntary_exit = VoluntaryExit {
        epoch: epoch.into(),
        validator_index,
    };
    let signed =
        generate_signed_voluntary_exit(&server.beacon, voluntary_exit, &keypair, &server.spec)
            .await?;
    let _ = insert_exit_message(
        &db,
        validator_index as i64,
        exit_message,
        &params.signature,
        &signed,
    )
    .await?;

    server
        .beacon
        .post_beacon_pool_voluntary_exits(&signed)
        .await
        .map_err(|err| anyhow!("Broadcast voluntary exit to beacon err: {err}"))?;

    Ok(Json(Response {}))
}

pub async fn generate_signed_voluntary_exit(
    client: &BeaconNodeHttpClient,
    voluntary_exit: VoluntaryExit,
    keypair: &Keypair,
    spec: &ChainSpec,
) -> anyhow::Result<SignedVoluntaryExit> {
    let genesis_data = client
        .get_beacon_genesis()
        .await
        .map_err(|err| anyhow!("Failed to get beacon genesis: {err}"))?
        .data;

    let fork = client
        .get_beacon_states_fork(eth2::types::StateId::Head)
        .await
        .map_err(|_| anyhow!("Failed  to get beacon states fork"))?
        .ok_or(anyhow!("Failed to get fork, state not found"))?
        .data;

    let signed = voluntary_exit.sign(
        &keypair.sk,
        &fork,
        genesis_data.genesis_validators_root,
        spec,
    );
    Ok(signed)
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use ethers::{types::Address, types::Signature};

    use super::ExitMessage;

    #[test]
    fn test_verify_signature() {
        let exit_message = ExitMessage(1000);
        let signature_str = "0xba6924c08932728a8ca330856ce3e092730314a4a969fea9352a30a5e5d2402a40e6c0353928706aae957867668854f5bc99db554bf06303b9340669ed3daa511b";
        let sig = Signature::from_str(signature_str).unwrap();

        let address_str = "0xA1151D1821704a4beB63e3f7dF6135327E9208e1";
        let address = Address::from_str(address_str).unwrap();
        sig.verify(exit_message.to_string(), address).unwrap()
    }
}
