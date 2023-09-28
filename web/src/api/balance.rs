use crate::utils::{caculate_arp, DEPOSIT_AMOUNT};

use super::*;
use eth2::types::Hash256;
use ethers::types::Address;
use storage::models::{
    query_el_fee_address_by_wc, select_validators_by_credentials,
    select_validator_cumulative_cl_reward, select_wc_cl_apr_7d,
};

#[derive(Debug, Deserialize)]
pub struct Params {
    wc: Hash256,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub total_balance: i64,
    pub effective_balance: i64,
    pub pending_protocol_balance: i64,
    pub total_rewards: i64,
    pub accumulative_protocol_reward: i64,
    pub accumulative_fee_reward: i64,
    pub total_apr: f64,
    pub cl_apr: f64,
    pub el_apr: f64,
    pub el_fee_address: Option<Address>,
}

pub async fn get_balance(
    Query(params): Query<Params>,
    State(server): State<Server>,
) -> Result<Json<Response>, AppError> {
    info!("Query wallet: {}", params.wc);
    let conn = server.pool.get().await?;
    let el_fee_address = query_el_fee_address_by_wc(&conn, &params.wc).await?;
    let validators = select_validators_by_credentials(&conn, params.wc).await?;
    let mut total_balance = 0;
    let mut effective_balance = 0;
    let mut pending_protocol_balance = 0;
    let mut accumulative_protocol_reward = 0;
    let el_apr = 0.0;

    let accumulative_fee_reward = 0;
    for validator in validators {
        total_balance += validator.amount as i64;
        if validator.data.is_some() {
            let data = validator.data.unwrap();
            pending_protocol_balance +=
                data.balance as i64 - (data.validator.effective_balance as i64);
            effective_balance += data.validator.effective_balance as i64;
            let protocol_reward = select_validator_cumulative_cl_reward(&conn, data.index).await? as i64;
            accumulative_protocol_reward += protocol_reward;
        } else {
            effective_balance += validator.amount as i64;
        }
    }
    let cl_apr = select_wc_cl_apr_7d(&conn, params.wc).await?;
    let total_apr = cl_apr + el_apr;
    let total_rewards = accumulative_fee_reward + accumulative_protocol_reward;
    Ok(Json(Response {
        total_balance,
        effective_balance,
        pending_protocol_balance,
        total_rewards,
        accumulative_protocol_reward,
        accumulative_fee_reward,
        total_apr,
        cl_apr,
        el_apr,
        el_fee_address,
    }))
}
