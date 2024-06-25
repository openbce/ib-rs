use libufm::{PartitionKey, UFMConfig, UFMError};

pub async fn run(conf: UFMConfig, pkey: &String, guids: &Vec<String>) -> Result<(), UFMError> {
    let ufm = libufm::connect(conf)?;

    let p = PartitionKey::try_from(pkey.clone())?;

    ufm.unbind_ports(p, guids.clone()).await?;

    Ok(())
}
