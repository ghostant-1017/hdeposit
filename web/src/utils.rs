use anyhow::Result;
use ethers::prelude::Abigen;
pub fn rust_file_generation() -> Result<()> {
    let abi_source = "../contract/abi/Deposit.abi";
    let out_file = "./deposit.rs";

    Abigen::new("DepositContract", abi_source)
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gen_abi() {
        rust_file_generation().unwrap()
    }
}
