use super::*;

#[derive(Debug, Deserialize)]
pub struct Params {
    // timespan: i64,
}

#[derive(Debug, Serialize)]
pub struct Response {
    total_apr: f64,
    consensus_apr: f64,
    execution_apr: f64,
}

pub async fn get_estimate_rewards(
    Query(_params): Query<Params>,
    State(server): State<Server>,
) -> Result<Json<Response>, AppError> {
    let db = server.pool.get().await?;
    let clock = server.clock;
    let slot = clock.now().unwrap();
    let epoch = slot.epoch(32);
    let end_epoch = epoch / 225 * 225;
    let start_epoch = end_epoch - 225 * 7;
    let sql = "select avg(t1.apr_7d)::DOUBLE PRECISION as avg_apr from
    (
        select validator_index, sum(reward_amount) / 32000000000 / 7 * 365 * 100 as apr_7d, count(epoch)
        from 
            protocol_reward
        where 
            epoch >= $1
        GROUP BY validator_index
    ) t1
        where t1.count = 7;";

    let row = db.query_one(sql, &[&(start_epoch.as_u64() as i64)]).await?;
    let consensus_apr: Option<f64> = row.get("avg_apr");
    let consensus_apr = consensus_apr.unwrap_or_default();
    let execution_apr = 0.0;
    let total_apr = consensus_apr + execution_apr;
    Ok(Json(Response {
        total_apr,
        consensus_apr,
        execution_apr,
    }))
}
