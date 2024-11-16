use libufm::{Partition, PartitionKey, PartitionQoS, UFMConfig, UFMError};

pub struct UpdateOptions {
    pub pkey: String,
    pub mtu: u16,
    pub ipoib: bool,
    pub service_level: u8,
    pub rate_limit: f64,
}

pub async fn run(conf: UFMConfig, opt: &UpdateOptions) -> Result<(), UFMError> {
    let ufm = libufm::connect(conf)?;

    let p = Partition {
        name: "".to_string(),
        pkey: PartitionKey::try_from(opt.pkey.clone())?,
        ipoib: opt.ipoib,
        qos: PartitionQoS {
            mtu_limit: opt.mtu,
            service_level: opt.service_level,
            rate_limit: opt.rate_limit,
        },
    };

    ufm.update_partition_qos(p).await?;

    Ok(())
}
