mod utils;

fn main() {
    println!("Hello, world!");
}
use anyhow::Result;
use eth2::BeaconNodeHttpClient;
pub async fn update_validators(client: &BeaconNodeHttpClient) -> Result<()> {
    // 1. Query from pg table `deposit_data`
    // 2. Query Validators from beacon node by pubkey
    // 3. Update pg table `validators`
    todo!()
}