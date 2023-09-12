use super::*;
use eth2::types::{Hash256, ValidatorData};
use storage::models::{select_validators_by_credentials, select_withdrawals_by_validator_index};

#[derive(Debug, Deserialize)]
pub struct Params {
    wc: Hash256,
}

#[derive(Serialize)]
pub struct HValidator {
    pub validator_data: ValidatorData,
    pub protocol_reward: u64,
}

pub async fn get_validators(
    Query(params): Query<Params>,
    State(server): State<Server>,
) -> Result<Json<Vec<HValidator>>, AppError> {
    todo!()
    // info!("Query validators: {}", params.wc);
    // let mut conn = server.pool.get().await?;
    // let tx = conn.transaction().await?;
    // let mut result = vec![];
    // let mut validators = select_validators_by_credentials(tx.client(), params.wc).await?;
    // validators.iter_mut().for_each(|validator| {
    //     validator.status = validator.status.superstatus();
    // });
    // for validator in validators {
    //     let protocol_reward: u64 =
    //         select_withdrawals_by_validator_index(tx.client(), validator.index)
    //             .await?
    //             .into_iter()
    //             .map(|withdrawl| withdrawl.amount)
    //             .sum();
    //     result.push(HValidator {
    //         validator_data: validator,
    //         protocol_reward,
    //     })
    // }

    // Ok(Json(result))
}
