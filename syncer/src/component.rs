use eth2::{types::ChainSpec, SensitiveUrl};

use crate::{beacon::BeaconClient, geth::Eth1Client};
use std::{sync::Arc, time::Duration};

use anyhow::anyhow;
use eth2::{
    types::{ConfigAndPreset, MainnetEthSpec},
    Timeouts,
};

use slot_clock::{SlotClock, SystemTimeSlotClock};

#[derive(Clone)]
pub struct EthComponent {
    pub beacon: Arc<BeaconClient>,
    pub eth1: Arc<Eth1Client>,

    pub clock: SystemTimeSlotClock,
    pub chain_spec: ChainSpec,
}

impl EthComponent {
    pub async fn new(eth1: &str, beacon: &str) -> anyhow::Result<Self> {
        let beacon = Arc::new(BeaconClient::new(
            SensitiveUrl::parse(beacon).map_err(|_| anyhow!("invalid eth2_endpoint"))?,
            Timeouts::set_all(Duration::from_secs(5)),
        ));
        let eth1 = Arc::new(ethers::providers::Provider::try_from(eth1)?);
        let config_and_preset: ConfigAndPreset = beacon
            .get_config_spec()
            .await
            .map_err(|_| anyhow!("get config from beacon"))?
            .data;
        let chain_spec = ChainSpec::from_config::<MainnetEthSpec>(config_and_preset.config())
            .ok_or(anyhow::anyhow!("from config"))?;
        let genesis_data = beacon
            .get_beacon_genesis()
            .await
            .map_err(|err| anyhow!("{err}"))?;
        let clock = slot_clock::SystemTimeSlotClock::new(
            chain_spec.genesis_slot,
            Duration::from_secs(genesis_data.data.genesis_time),
            Duration::from_secs(chain_spec.seconds_per_slot),
        );
        Ok(Self {
            beacon,
            eth1,
            clock,
            chain_spec,
        })
    }
}
