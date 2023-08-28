use std::env;

use ethers::{signers::LocalWallet, utils::hex};
use k256::elliptic_curve::generic_array::GenericArray;

pub fn inital_wallet_from_env() -> anyhow::Result<LocalWallet> {
    let secret_key = env::var("CONTRACT_OWNER_KEY")?;
    let key_hex = hex::decode(secret_key)?;
    let key = k256::SecretKey::from_bytes(&GenericArray::clone_from_slice(&key_hex))?;
    let wallet = key.into();
    Ok(wallet)
}
