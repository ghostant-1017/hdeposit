use super::*;
use eth2::types::{Hash256, ValidatorData};
use storage::models::select_validators_by_credentials;

#[derive(Debug, Deserialize)]
pub struct Params {
    wc: Hash256,
}

pub async fn get_validators(
    Query(params): Query<Params>,
    State(server): State<Server>,
) -> Result<Json<Vec<ValidatorData>>, AppError> {
    info!("Query validators: {}", params.wc);
    let conn = server.pool.get().await?;
    let mut validators = select_validators_by_credentials(&conn, params.wc).await?;
    validators.iter_mut().for_each(|validator| {
        validator.status = validator.status.superstatus();
    });
    Ok(Json(validators))
}
