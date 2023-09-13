mod syncer;
pub use syncer::*;
mod processor;
pub use processor::*;
mod feemgr;
pub use feemgr::*;

use ethers::providers::Provider;
use crate::vault::Vault;
use ethers::prelude::Http;
use ethers::prelude::SignerMiddleware;
use ethers::signers::Wallet;
use k256::ecdsa::SigningKey;
pub type VaultContract = Vault<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;
