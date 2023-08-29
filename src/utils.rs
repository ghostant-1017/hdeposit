use anyhow::Result;
use ethers::prelude::Abigen;
pub fn rust_file_generation() -> Result<()> {
    let abi_source = "./abi/Vault.abi";
    let out_file = "./Vault.rs";

    Abigen::new("Vault", abi_source)
        .unwrap()
        .add_derive("serde::Serialize")
        .unwrap()
        .add_derive("serde::Deserialize")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file(out_file)
        .unwrap();
    Ok(())
}

use ethers::types::Bytes;
use lighthouse_bls::{Hash256, Keypair, SignatureBytes};
use lighthouse_types::{ChainSpec, DepositData, DepositMessage, SignedRoot};

pub fn generate_deposit_data(
    spec: &ChainSpec,
    kp: &Keypair,
    withdrawal_credential: &Bytes,
    amount: u64,
) -> Result<DepositData> {
    // let seckey = kp.sk.clone();
    let pubkey = kp.pk.clone();
    let dm = DepositMessage {
        pubkey: pubkey.into(),
        withdrawal_credentials: Hash256::from_slice(withdrawal_credential),
        amount,
    };
    let domain = spec.get_deposit_domain();
    let msg = dm.signing_root(domain);

    let deposit_data = DepositData {
        pubkey: dm.pubkey,
        withdrawal_credentials: dm.withdrawal_credentials,
        amount: dm.amount,
        signature: SignatureBytes::from(kp.sk.sign(msg)),
    };
    Ok(deposit_data)
}

#[cfg(test)]
mod tests {
    use super::rust_file_generation;

    #[test]
    fn test_gen_abi() {
        rust_file_generation().unwrap()
    }
}
