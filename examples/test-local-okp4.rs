//! Deploys Abstract and the App module to a local Junod instance. See how to spin up a local chain here: https://docs.junonetwork.io/developer-guides/junod-local-dev-setup
//! You can also start a juno container by running `just juno-local`.
//!
//! Ensure the local juno is running before executing this script.
//!
//! # Run
//!
//! `cargo run --example test-local`

use abstract_core::objects::gov_type::GovernanceDetails;
use abstract_interface::{Abstract, AppDeployer, VCExecFns};
use app::{
    contract::{APP_ID, APP_VERSION},
    msg::AppInstantiateMsg,
    AppInterface,
};
use cw_orch::{
    anyhow,
    deploy::Deploy,
    prelude::{Daemon, TxHandler},
    tokio::runtime::Runtime, daemon::{ChainInfo, ChainKind, NetworkInfo},
};
use semver::Version;
use speculoos::{assert_that, prelude::BooleanAssertions};

const LOCAL_MNEMONIC: &str = "island position immense mom cross enemy grab little deputy tray hungry detect state helmet tomorrow trap expect admit inhale present vault reveal scene atom";

pub const OKP4_NETWORK: NetworkInfo = NetworkInfo {
    id: "okp4-localnet",
    pub_address_prefix: "okp4",
    coin_type: 118u32, // TBD
};

pub const LOCAL_OKP4: ChainInfo = ChainInfo {
    kind: ChainKind::Local,
    chain_id: "okp4-localnet",
    gas_denom: "uknow",
    gas_price: 0.0,
    grpc_urls: &["http://localhost:9090"],
    network_info: OKP4_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let version: Version = APP_VERSION.parse().unwrap();
    let runtime = Runtime::new()?;

    let daemon = Daemon::builder()
        .chain(LOCAL_OKP4)
        .mnemonic(LOCAL_MNEMONIC)
        .handle(runtime.handle())
        .build()
        .unwrap();

    // detect if they'ee already loaded somehow

    // Deploy abstract locally
    // let abstract_deployment = Abstract::deploy_on(daemon.clone(), daemon.sender().to_string())?;
    let abstract_deployment = Abstract::load_from(daemon.clone())?;

    let app = AppInterface::new(APP_ID, daemon.clone());

    // Create account
    let account = abstract_deployment.account_factory.create_default_account(
        GovernanceDetails::Monarchy {
            monarch: daemon.sender().into_string(),
        },
    )?;
    
    // Claim namespace
    abstract_deployment
        .version_control
        .claim_namespace(account.id()?, format!("my-namespace{}", account.id()?))?;

    // Deploy
    app.deploy(version)?;

    // Install app
    account.install_app(app, &AppInstantiateMsg {}, None)?;

    assert_that!(account.manager.is_module_installed(APP_ID).unwrap()).is_true();
    Ok(())
}
