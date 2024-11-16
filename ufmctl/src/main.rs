use clap::{Parser, Subcommand};

use libufm::{UFMCert, UFMConfig, UFMError};

mod bind;
mod create;
mod delete;
mod info;
mod list;
mod unbind;
mod update;
mod version;
mod view;

#[derive(Parser)]
#[command(name = "ufmctl")]
#[command(author = "Klaus Ma <klaus@xflops.cn>")]
#[command(version = "0.1.0")]
#[command(about = "UFM command line", long_about = None)]
struct Options {
    #[clap(long, env = "UFM_ADDRESS")]
    ufm_address: Option<String>,
    #[clap(long, env = "UFM_USERNAME")]
    ufm_username: Option<String>,
    #[clap(long, env = "UFM_PASSWORD")]
    ufm_password: Option<String>,
    #[clap(long, env = "UFM_TOKEN")]
    ufm_token: Option<String>,
    #[clap(long, env = "UFM_CA_CRT")]
    ufm_ca_crt: Option<String>,
    #[clap(long, env = "UFM_TLS_KEY")]
    ufm_tls_key: Option<String>,
    #[clap(long, env = "UFM_TLS_CRT")]
    ufm_tls_crt: Option<String>,
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// View the detail of the partition
    View {
        /// The pkey of the partition to view
        #[arg(short, long)]
        pkey: String,
    },
    /// List all partitions
    List,
    /// Get the version of UFM
    Version,
    /// Get the configuration information of UFM
    Info,
    /// Delete the partition
    Delete {
        /// The pkey of the partition to delete
        #[arg(short, long)]
        pkey: String,
    },
    /// Create a partition
    Create {
        /// The pkey for the new partition
        #[arg(short, long)]
        pkey: String,
        /// The IPOverIB of the new partition
        #[arg(long, default_value_t = true)]
        ipoib: bool,
        /// The Index0 of the new partition
        #[arg(long, default_value_t = true)]
        index0: bool,
        /// The Membership of the new partition
        #[arg(short, long, default_value_t = String::from("full"))]
        membership: String,
        /// The GUIDs of the new partition
        #[arg(short, long)]
        guids: Vec<String>,
        /// The MTU of the new partition
        #[arg(long, default_value_t = 4)]
        mtu: u16,
        /// The ServiceLevel of the new partition
        #[arg(short, long, default_value_t = 0)]
        service_level: u8,
        /// The RateLimit of the new partition
        #[arg(short, long, default_value_t = 100f64)]
        rate_limit: f64,
    },
    /// Update the partition
    Update {
        /// The pkey for the new partition
        #[arg(short, long)]
        pkey: String,
        /// The IPOverIB of the new partition
        #[arg(long, default_value_t = true)]
        ipoib: bool,
        /// The MTU of the new partition
        #[arg(long, default_value_t = 4)]
        mtu: u16,
        /// The ServiceLevel of the new partition
        #[arg(short, long, default_value_t = 0)]
        service_level: u8,
        /// The RateLimit of the new partition
        #[arg(short, long, default_value_t = 100f64)]
        rate_limit: f64,
    },

    /// Bind ports to the partition
    Bind {
        /// The pkey of the partition
        #[arg(short, long)]
        pkey: String,
        /// A list of GUID to bind
        #[arg(short, long)]
        guids: Vec<String>,
    },

    /// Unbind ports from the partition
    Unbind {
        /// The pkey of the partition
        #[arg(short, long)]
        pkey: String,
        /// A list of GUID to unbind
        #[arg(short, long)]
        guids: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), UFMError> {
    env_logger::init();

    let opt: Options = Options::parse();

    let conf = load_conf(&opt);
    match &opt.command {
        Some(Commands::Delete { pkey }) => delete::run(conf, pkey).await?,
        Some(Commands::Version) => version::run(conf).await?,
        Some(Commands::Info) => info::run(conf).await?,
        Some(Commands::List) => list::run(conf).await?,
        Some(Commands::View { pkey }) => view::run(conf, pkey).await?,
        Some(Commands::Bind { pkey, guids }) => bind::run(conf, pkey, guids).await?,
        Some(Commands::Unbind { pkey, guids }) => unbind::run(conf, pkey, guids).await?,
        Some(Commands::Update {
            pkey,
            mtu,
            ipoib,
            service_level,
            rate_limit,
        }) => {
            let opt = update::UpdateOptions {
                pkey: pkey.to_string(),
                mtu: *mtu,
                service_level: *service_level,
                rate_limit: *rate_limit,
                ipoib: *ipoib,
            };
            update::run(conf, &opt).await?
        }

        Some(Commands::Create {
            pkey,
            ipoib,
            index0,
            membership,
            guids,
            mtu,
            service_level,
            rate_limit,
        }) => {
            let opt = create::CreateOptions {
                pkey: pkey.to_string(),
                ipoib: *ipoib,
                index0: *index0,
                membership: membership.to_string(),
                guids: guids.to_vec(),
                mtu: *mtu,
                service_level: *service_level,
                rate_limit: *rate_limit,
            };
            create::run(conf, &opt).await?
        }
        None => {}
    };

    Ok(())
}

fn load_conf(opt: &Options) -> UFMConfig {
    let ufm_address = match opt.ufm_address.clone() {
        Some(s) => s,
        None => panic!("UFM_ADDRESS environment or ufm_address parameter not found"),
    };

    let cert = if opt.ufm_ca_crt.is_some() && opt.ufm_tls_key.is_some() && opt.ufm_tls_crt.is_some()
    {
        Some(UFMCert {
            ca_crt: opt.ufm_ca_crt.clone().unwrap(),
            tls_key: opt.ufm_tls_key.clone().unwrap(),
            tls_crt: opt.ufm_tls_crt.clone().unwrap(),
        })
    } else {
        None
    };

    UFMConfig {
        address: ufm_address,
        username: opt.ufm_username.clone(),
        password: opt.ufm_password.clone(),
        token: opt.ufm_token.clone(),
        cert,
    }
}
