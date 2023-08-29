use anyhow::Result;
use ethers::prelude::Abigen;
pub fn rust_file_generation() -> Result<()> {
    let abi_source = "./abi/Vault.abi";
    let out_file = "./test.out";

    Abigen::new("Vault", abi_source)
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file(out_file)
        .unwrap();
    Ok(())
}

use ethers::types::Bytes;
use lighthouse_bls::{Keypair, Hash256, SignatureBytes};
use lighthouse_types::{DepositMessage, ChainSpec, SignedRoot, DepositData};

pub fn generate_deposit_data(spec: &ChainSpec, kp: &Keypair, withdrawal_credential: &Bytes, amount: u64) -> Result<DepositData> {
    let seckey = kp.sk;
    let pubkey = kp.pk;
    let dm = DepositMessage {
        pubkey: pubkey.into(),
        withdrawal_credentials: Hash256::from_slice(&withdrawal_credential),
        amount,
    };
    let domain = spec.get_deposit_domain();
    let msg = dm.signing_root(domain);

    let deposit_data = DepositData {
        pubkey: dm.pubkey,
        withdrawal_credentials: dm.withdrawal_credentials,
        amount: dm.amount,
        signature: SignatureBytes::from(seckey.sign(msg))
    };
    Ok(deposit_data)
}
