use abstract_core::objects::{gov_type::GovernanceDetails, AccountId};
use abstract_interface::{Abstract, AbstractAccount, AppDeployer, VCExecFns};
use app::{
    contract::{APP_ID, APP_VERSION},
    msg::{AppInstantiateMsg, ConfigResponse},
    *,
};
// Use prelude to get all the necessary imports
use cw_orch::{anyhow, deploy::Deploy, prelude::*, daemon::{NetworkInfo, ChainInfo, ChainKind}, tokio::runtime::Runtime};

// use cosmwasm_std::Addr;

// consts for testing
const ADMIN: &str = "admin";
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

/// Set up the test environment with the contract installed
fn setup() -> anyhow::Result<(AbstractAccount<Daemon>, Abstract<Daemon>, AppInterface<Daemon>)> {
    // Create a sender
    // let sender = Addr::unchecked(ADMIN);

    let runtime = Runtime::new()?;
    let daemon = Daemon::builder()
        .chain(LOCAL_OKP4)
        .mnemonic(LOCAL_MNEMONIC)
        .handle(runtime.handle())
        .build()
        .unwrap();


    // Construct the counter interface
    // let app = AppInterface::new(APP_ID, mock.clone());
    let app = AppInterface::new(APP_ID, daemon.clone());

    // Deploy Abstract to the mock
    // let abstr_deployment = Abstract::deploy_on(mock, sender.to_string())?;
    let abstract_deployment = Abstract::deploy_on(daemon.clone(), daemon.sender().to_string())?;
    // let abstract_deployment = Abstract::load_from(daemon.clone())?;


    // Create a new account to install the app onto
    let account =
    abstract_deployment
            .account_factory
            .create_default_account(GovernanceDetails::Monarchy {
                monarch: ADMIN.to_string(),
            })?;

    // claim the namespace so app can be deployed
    abstract_deployment
        .version_control
        .claim_namespace(AccountId::local(1), "my-namespace".to_string())?;

    app.deploy(APP_VERSION.parse()?)?;

    account.install_app(app.clone(), &AppInstantiateMsg {}, None)?;

    Ok((account, abstract_deployment, app))
}

#[test]
fn successful_install() -> anyhow::Result<()> {
    // Set up the environment and contract
    let (_account, _abstr, app) = setup()?;

    let config = app.config()?;
    assert_eq!(config, ConfigResponse {});
    Ok(())
}
