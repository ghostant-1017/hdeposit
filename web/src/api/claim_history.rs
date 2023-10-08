use std::ops::Div;

use eth2::types::Hash256;
use ethers::types::H256;
use storage::models::{query_el_fee_address_by_wc, select_claim_by_address};

use super::*;

#[derive(Debug, Deserialize)]
pub struct Params {
    wc: Hash256,
}

#[derive(Debug, Serialize)]
pub struct ClaimInfo {
    block_number: u64,
    tx_hash: H256,
    amount: u64,
}

#[derive(Debug, Serialize)]
pub struct Response {
    data: Vec<ClaimInfo>,
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
        None => return Ok(Json(Response { data: vec![] })),
    };
    // 2. query el_fee_contract logs
    let data = select_claim_by_address(&db, address)
        .await?
        .into_iter()
        .map(|(log, meta)| ClaimInfo {
            block_number: meta.block_number.as_u64(),
            tx_hash: meta.transaction_hash,
            amount: log.user_amount.div(1000000000).as_u64(),
        })
        .collect();
    Ok(Json(Response { data }))
}
