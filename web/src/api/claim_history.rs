use contract::elfee::SplitFeeFilter;
use ethers::prelude::LogMeta;
use eth2::types::Hash256;
use storage::models::{query_el_fee_address_by_wc, select_claim_by_address};

use super::*;

#[derive(Debug, Deserialize)]
pub struct Params {
    wc: Hash256
}

#[derive(Debug, Serialize)]
pub struct Response {
    data: Vec<(SplitFeeFilter, LogMeta)>
}

pub async fn get_claim_history(
    Query(params): Query<Params>,
    State(server): State<Server>,
) -> Result<Json<Response>, AppError> {
    let db = server.pool.get().await?;
     // 1. select wc related el_fee_contract address
    let address = query_el_fee_address_by_wc(&db, &params.wc).await?;
    let address = match address {
        Some(address) => address,
        None => {
            return Ok(Json(Response {
                data: vec![]
            }))
        } 
    };
     // 2. query el_fee_contract logs
    let data = select_claim_by_address(&db, address).await?;
    Ok(Json(Response {
        data
    }))
}