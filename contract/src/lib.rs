pub mod deposit;
pub mod vault;
pub mod elfee;
use anyhow::Result;
use ethers::prelude::Abigen;

pub fn rust_file_generation() -> Result<()> {
    let abi_source = "./abi/Deposit.abi";
    let out_file = "./src/deposit.rs";
    generate("DepositContract", abi_source, out_file);

    let abi_source = "./abi/Vault.abi";
    let out_file = "./src/vault.rs";
    generate("Vault", abi_source, out_file);

    let abi_source = "./abi/ELFee.abi";
    let out_file = "./src/elfee.rs";
    generate("ELFee", abi_source, out_file);

    Ok(())
}

fn generate(contract_name: &str, abi_source: &str, out_file: &str) {
    Abigen::new(contract_name, abi_source)
    .unwrap()
    .add_derive("serde::Serialize")
    .unwrap()
    .add_derive("serde::Deserialize")
    .unwrap()
    .generate()
    .unwrap()
    .write_to_file(out_file)
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gen_abi() {
        rust_file_generation().unwrap()
    }
}
