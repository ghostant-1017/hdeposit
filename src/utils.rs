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
