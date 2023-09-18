use bb8_postgres::tokio_postgres::Client;
use ethers::types::Signature as Eth1Signature;
use lighthouse_types::{signed_voluntary_exit, SignedVoluntaryExit};

// pub struct StoredExitMessage {
//     pk: i64,
//     validator_index: i64,
//     raw_message: String,
//     signature: Eth1Signature,
// }

pub async fn insert_exit_message(
    client: &Client,
    validator_index: i64,
    user_message: String,
    user_signature: &Eth1Signature,
    signed_voluntary_exit: &SignedVoluntaryExit,
) -> anyhow::Result<bool> {
    let sql = "insert into exit_messages(validator_index, user_message, user_signature, signed_voluntary_exit) 
        values($1, $2, $3, $4) on conflict(validator_index) do nothing;";
    let user_signature = serde_json::to_string(&user_signature)?;
    let signed_voluntary_exit = serde_json::to_value(&signed_voluntary_exit)?;
    let result = client
        .execute(
            sql,
            &[
                &validator_index,
                &user_message,
                &user_signature,
                &signed_voluntary_exit,
            ],
        )
        .await?;
    if result == 0 {
        return Ok(false);
    }
    Ok(true)
}
