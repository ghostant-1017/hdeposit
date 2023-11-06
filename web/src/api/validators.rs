

use super::*;

use bb8_postgres::tokio_postgres::Client;
use eth2::types::{Hash256, ValidatorData, ValidatorStatus};
use ethers::types::H256;
use storage::models::{
    select_validator_cl_apr_7d, select_validator_cumulative_cl_reward,
    select_validators_by_credentials, HellmanValidator,
    select_validator_el_apr_7d,
};

// 365 * 24 * 3600 / 12 / 32

#[derive(Debug, Deserialize)]
pub struct Params {
    wc: Hash256,
}

#[derive(Serialize)]
pub struct ValidatorInfo {
    pub index: Option<u64>,
    pub balance: u64,
    pub status: ValidatorStatus,
    pub withdrawal_credentials: H256,
    pub accumulative_protocol_reward: u64,
    pub cl_apr: f64,
    pub el_apr: f64,
    pub validator_data: Option<ValidatorData>,
}

impl ValidatorInfo {
    pub async fn new(
        client: &Client,
        validator: HellmanValidator,
        _clock: &SystemTimeSlotClock,
    ) -> anyhow::Result<Self> {
        if validator.data.as_ref().is_none() {
            Ok(Self {
                index: validator.index,
                balance: validator.amount,
                withdrawal_credentials: validator.withdrawal_credentials,
                status: ValidatorStatus::Pending,
                accumulative_protocol_reward: 0,
                cl_apr: 0.0,
                el_apr: 0.0,
                validator_data: None,
            })
        } else {
            let validator_data = validator.data.unwrap();
            let accumulative_protocol_reward =
                select_validator_cumulative_cl_reward(client, validator_data.index).await?;
            let cl_apr = select_validator_cl_apr_7d(client, validator_data.index).await?;
            let el_apr = select_validator_el_apr_7d(client, validator_data.index as i64).await?;
            Ok(Self {
                index: validator.index,
                withdrawal_credentials: validator.withdrawal_credentials,
                balance: validator.amount,
                status: validator_data.status,
                accumulative_protocol_reward,
                cl_apr,
                el_apr,
                validator_data: Some(validator_data),
            })
        }
    }
}

pub async fn get_validators(
    Query(params): Query<Params>,
    State(server): State<Server>,
) -> Result<Json<Vec<ValidatorInfo>>, AppError> {
    info!("Query validators: {}", params.wc);
    let mut conn = server.pool.get().await?;
    let tx = conn.transaction().await?;
    let mut result = vec![];
    let validators = select_validators_by_credentials(tx.client(), params.wc).await?;
    for validator in validators {
        let info = ValidatorInfo::new(tx.client(), validator, &server.clock).await?;
        result.push(info)
    }

    Ok(Json(result))
}
