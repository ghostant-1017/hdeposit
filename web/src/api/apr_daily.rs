use std::collections::HashMap;

use eth2::types::Epoch;
use storage::models::{select_max_epoch, select_range_validators_count, select_range_cl_rewards, select_range_el_rewards};

use super::{*, utils::epoch_to_timestamp};

#[derive(Debug, Deserialize)]
pub struct Params {
}

#[derive(Debug, Serialize)]
pub struct Response {
    data: Vec<APRDaily>
}

#[derive(Debug, Serialize)]
pub struct APRDaily {
    unix: u64,
    total_apr: f64,
    consensus_apr: f64,
    execution_apr: f64,
}

pub async fn get_apr_daily(
    Query(_params): Query<Params>,
    State(server): State<Server>,
) -> Result<Json<Response>, AppError> {
    let mut db = server.pool.get().await?;
    let clock = server.clock;
    let tx = db.transaction().await?;
    let to = select_max_epoch(tx.client()).await? as i64;
    let from = to - 225 * 6 as i64;
    let counts = select_range_validators_count(tx.client(), from, to).await?;
    let cl_rewards: HashMap<Epoch, u64> = select_range_cl_rewards(tx.client(), from, to)
    .await?
    .into_iter()
    .collect();
    let el_rewards: HashMap<Epoch, u64> = select_range_el_rewards(tx.client(), from, to).await?
    .into_iter()
    .collect();
    drop(tx);
    let mut data = vec![];
    for (epoch, count) in counts {
        let cl_reward = *cl_rewards.get(&epoch).unwrap_or(&0) as f64;
        let el_reward = *el_rewards.get(&epoch).unwrap_or(&0) as f64;
        let cl_apr = cl_reward / (count * 32_000_000_000) as f64 * 365.0 * 100.0 ;
        let el_apr = el_reward / (count * 32_000_000_000) as f64 * 365.0 * 100.0 ;
        let total_apr = cl_apr + el_apr;
        data.push(APRDaily {
            unix: epoch_to_timestamp(&clock, epoch.as_u64())?,
            total_apr,
            consensus_apr: cl_apr,
            execution_apr: el_apr,
        }) 
    }
    Ok(Json(Response { data }))
}
