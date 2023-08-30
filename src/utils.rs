use std::collections::HashMap;

use anyhow::Result;
use bytes::BufMut;
use ethers::prelude::Abigen;
use ethers::types::Bytes as EBytes;
use bytes::Bytes;
use bytes::BytesMut;
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

use lighthouse_bls::{Hash256, Keypair, SignatureBytes};
use lighthouse_types::{ChainSpec, DepositData, DepositMessage, SignedRoot};
use tree_hash::TreeHash;

pub fn generate_deposit_data(
    spec: &ChainSpec,
    kp: &Keypair,
    withdrawal_credential: &EBytes,
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


pub fn generate_deposit_calldata(batch: Vec<DepositData>) -> (EBytes, EBytes, Vec<[u8;32]>, Bytes, Vec<u32>) {
    let mut hm: HashMap<Hash256, Vec<DepositData>> = HashMap::new();
    // Group by withdrawl_credentials
    for deposit_data in batch {
        let wc = deposit_data.withdrawal_credentials.clone();
        hm.entry(wc).and_modify(|v| v.push(deposit_data.clone())).or_insert(vec![deposit_data]);
    }

    let mut pubkeys = BytesMut::new();
    let mut signatures = BytesMut::new();
    let mut deposit_data_roots = vec![];
    let mut withdrawal_credentials = BytesMut::new();
    let mut ns = vec![];
    for (wc, batch) in hm { 
        batch
        .iter()
        .for_each(|dd| {
            pubkeys.put(Bytes::copy_from_slice(&dd.pubkey.serialize()));
            signatures.put(Bytes::copy_from_slice(&dd.signature.serialize()));
            deposit_data_roots.push(dd.tree_hash_root().to_fixed_bytes());
        });
        withdrawal_credentials.put(Bytes::copy_from_slice(wc.as_fixed_bytes()));
        ns.push(batch.len() as u32);
    }
    let pubkeys = pubkeys.freeze();
    let signatures = signatures.freeze();
    let withdrawal_credentials = withdrawal_credentials.freeze();

    (pubkeys.into(), signatures.into(), deposit_data_roots, withdrawal_credentials.into(), ns)
}

#[cfg(test)]
mod tests {
    use super::rust_file_generation;

    #[test]
    fn test_gen_abi() {
        rust_file_generation().unwrap()
    }

    #[test]
    fn test_generate_calldata() {
        
    }
}
