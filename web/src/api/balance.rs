use crate::utils::{DEPOSIT_AMOUNT, caculate_arp};

use super::*;
use eth2::types::Hash256;
use storage::models::{select_validators_by_credentials, select_withdrawals_by_validator_index};

#[derive(Debug, Deserialize)]
pub struct Params {
    wc: Hash256,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub total_balance: i64,
    pub effective_balance: i64,
    pub pending_protocol_balance: i64,

    pub accumulative_protocol_reward: i64,
    pub accumulative_fee_reward: i64,
    
    pub cl_arp: f64,
}

pub async fn get_balance(
    Query(params): Query<Params>,
    State(server): State<Server>,
) -> Result<Json<Response>, AppError> {
    info!("Query wallet: {}", params.wc);
    let conn = server.pool.get().await?;
    let validators = select_validators_by_credentials(&conn, params.wc).await?;
    let mut total_balance = 0;
    let mut effective_balance = 0;
    let mut pending_protocol_balance = 0;
    let mut accumulative_protocol_reward = 0;
    let mut accumulative_fee_reward = 0;
    let mut cl_arps = vec![];
    let mut cl_arp = 0.0;
    for validator in validators {
        total_balance += validator.amount as i64;
        if validator.data.is_some() {
            let data = validator.data.unwrap();
            pending_protocol_balance += data.balance as i64 - (data.validator.effective_balance as i64);
            effective_balance += data.validator.effective_balance as i64;
            let protocol_reward = select_withdrawals_by_validator_index(&conn, validator.index)
            .await?
            .into_iter()
            .map(|w| w.amount as i64)
            .filter(|amount| *amount < DEPOSIT_AMOUNT as i64)
            .sum::<i64>();
            accumulative_protocol_reward += protocol_reward;
            let arp = caculate_arp(&server.clock, data.validator.activation_epoch.as_u64(), protocol_reward as u64)?;
            cl_arps.push(arp);
        }else {
            effective_balance += validator.amount as i64;
        }
    }
    if cl_arps.len() != 0 {
        cl_arp = cl_arps.iter().sum::<f64>() / cl_arps.len() as f64;
    }
    Ok(Json(Response {
        total_balance,
        effective_balance,
        pending_protocol_balance,
        accumulative_protocol_reward,
        accumulative_fee_reward,
        cl_arp,
    }))
}
