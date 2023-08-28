use crate::vault::PreDepositFilter;
use anyhow::Result;
use ethers::prelude::LogMeta;
use lighthouse_types::DepositData;
use lighthouse_bls::PublicKeyBytes;
use lighthouse_bls::SecretKey;

pub struct ProcessorService {}

impl ProcessorService {
    pub fn new() -> Self {
        // let mut conn = pool.get().await?;
        // let ks = query_unused_key_store(&mut conn).await?;
        // let kp = ks.decrypt_keypair("Ipfs@111".as_bytes()).map_err(|err| anyhow!("decrypt"))?;
        // let data = DepositData {pubkey:kp.pk.into(), withdrawal_credentials: todo!(), amount: todo!(), signature: todo!() };
        // info!("{:?}", kp);
        todo!()
    }

    pub fn start_update_service(self) -> Result<()> {
        tokio::spawn(async move {
            match self.do_update().await {
                Ok(_) => todo!(),
                Err(_err) => todo!(),
            }
        });
        Ok(())
    }

    async fn do_update(&self) -> Result<()> {
        todo!()
    }

    // Asc by block height and log index
    async fn select_pending_logs(&self) -> Result<Vec<(PreDepositFilter, LogMeta)>> {
        todo!()
    }

    async fn call_contract_deposit(&self, logs: Vec<(PreDepositFilter, LogMeta)>) {
        todo!()
    }
}

pub fn generate_calldata() {}
