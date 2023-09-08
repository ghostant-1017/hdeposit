use super::*;
use eth2::types::Hash256;
use storage::models::select_validators_by_credentials;

#[derive(Debug, Deserialize)]
pub struct Params {
    wc: Hash256,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub total_balance: i64,
    pub effective_balance: i64,
    pub pending_protocol_balance: i64,
    pub annual_rewards_rate: f64,
}

pub async fn get_balance(
    Query(params): Query<Params>,
    State(server): State<Server>,
) -> Result<Json<Response>, AppError> {
    info!("Query balace: {}", params.wc);
    let conn = server.pool.get().await?;
    let validators = select_validators_by_credentials(&conn, params.wc).await?;
    // TODO: add utils method for current epoch
    let _current_epoch = server.clock.now().unwrap() / 32;
    let _epoch_per_year = 82125;
    let mut total_balance = 0;
    let mut effective_balance = 0;
    let mut pending_protocol_balance = 0;
    // let mut rates = 0.0;
    for validator in validators {
        total_balance += validator.balance as i64;
        effective_balance += validator.validator.effective_balance as i64;
        pending_protocol_balance +=
            (validator.balance as i64) - (validator.validator.effective_balance as i64);
    }
    Ok(Json(Response {
        total_balance,
        effective_balance,
        pending_protocol_balance,
        annual_rewards_rate: 0.0,
    }))
}