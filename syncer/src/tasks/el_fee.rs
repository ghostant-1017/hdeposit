use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Add;
use std::ops::AddAssign;
use std::sync::Arc;

use anyhow::anyhow;
use eth2::types::Epoch;
use eth2::types::Slot;
use crate::beacon::BeaconClient;
use crate::geth::Eth1Client;
use crate::geth::get_block_receipts_by_hash;
use storage::models::ELFee;
use eth2::types::{BeaconBlock, EthSpec};
use ethers::types::TransactionReceipt;


pub struct AttestationReward {
    epoch: u64,
    validator_index: u64,
    amount: u64,
}

pub struct SyncCommitteeReward { 
    epoch: u64,
    validator_index: u64,
    amount: u64,
}

pub struct BlockReward {
    slot: u64,

}

pub async fn extract_rewards(beacon: Arc<BeaconClient>, from: Slot, to: Slot, validators: &HashSet<u64>) -> anyhow::Result<()> {
    let block_rewards = beacon
    .get_lighthouse_analysis_block_rewards(from, to)
    .await
    .map_err(|err| anyhow!("get block_rewards: {err}"))?;

    let mut epoch_rewards = HashMap::<Epoch, HashMap<u64, i64>>::new();
    for block_reward in block_rewards {
        let epoch = block_reward.meta.slot.epoch(32);
        let proposer_index = block_reward.meta.proposer_index;
        let sync_committee_rewards = block_reward.sync_committee_rewards;
        let attestation_rewards = block_reward.attestation_rewards;
        if validators.contains(&proposer_index) {
            epoch_rewards.entry(epoch)
            .or_default().entry(proposer_index)
            .or_default().add_assign(sync_committee_rewards as i64);
        }
        for attestation_reward in attestation_rewards.per_attestation_rewards {
            for validator in validators {
                if attestation_reward.contains_key(validator) {
                    let amount = *attestation_reward.get(validator).unwrap();
                    epoch_rewards.entry(epoch).or_default().entry(*validator).or_default().add_assign(amount as i64)
                }
            }
        }
    }
    println!("{:?}", epoch_rewards);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use eth2::{BeaconNodeHttpClient, SensitiveUrl, Timeouts};

    use super::*;
    #[tokio::test]
    async fn test_epoch_rewards() {
        let eth2_endpoint = "http://localhost:5052/";
        let url = SensitiveUrl::parse(eth2_endpoint).unwrap();
        let beacon = BeaconNodeHttpClient::new(url,Timeouts::set_all(Duration::from_secs(5)));
        let mut validators = HashSet::new();
        validators.insert(119812 as u64);
        extract_rewards(Arc::new(beacon), Slot::new(6571904), Slot::new(6571935), &validators).await.unwrap();
    }
}

pub async fn extract_el_rewards_capella<T: EthSpec>(block: BeaconBlock<T>, eth1: &Eth1Client) -> anyhow::Result<ELFee> {
    let block = block.as_capella().map_err(|_| anyhow!("not capella block"))?;
    let slot = block.slot.as_u64();
    let validator_index = block.proposer_index;

    let fee_recipient = block.body.execution_payload.execution_payload.fee_recipient;
    let block_number = block.body.execution_payload.execution_payload.block_number;
    let block_hash = block.body.execution_payload.execution_payload.block_hash;
    // 3. query block_hash from eth1
    let receipts = get_block_receipts_by_hash(eth1, block_number).await?;
    let amount = caculate_block_fee(receipts)?;
    Ok(ELFee {
        slot,
        block_number,
        block_hash: block_hash.into_root(),
        validator_index,
        fee_recipient,
        amount
    })
}

fn caculate_block_fee(receipts: Vec<TransactionReceipt>) -> anyhow::Result<u64> {
    let mut total = 0;
    for receipt in receipts {
        let gas_price = receipt.effective_gas_price.ok_or(anyhow!("effective_gas_price not found"))?.as_u64();
        let gas_used = receipt.gas_used.ok_or(anyhow!("gas_used not found"))?.as_u64();
        total += gas_price * gas_used;
    }
    Ok(total)
}