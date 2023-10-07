use abstract_core::objects::{gov_type::GovernanceDetails, AccountId};
use abstract_interface::{Abstract, AbstractAccount, AppDeployer, VCExecFns};
use app::{
    contract::{APP_ID, APP_VERSION},
    msg::{AppInstantiateMsg, ConfigResponse},
    *,
};
use cw_orch::{anyhow, deploy::Deploy, prelude::*, daemon::{NetworkInfo, ChainInfo, ChainKind}, tokio::runtime::Runtime};


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

fn setup_okp4_contracts() -> anyhow::Result<()> { // TBD
    // deploy contracts and return them??
    env_logger::init();
    dotenv::dotenv().unwrap();

    let sender = Addr::unchecked("sender");
    
    let rt = Runtime::new().unwrap();
    let network = LOCAL_OKP4;
    let chain = DaemonBuilder::default()
        .handle(rt.handle())
        .chain(network)
        .build().unwrap();

        let cw20 = Cw20Base::new("cw-plus:cw20", chain.clone());

        cw20.upload()?;
        cw20.instantiate(&InstantiateMsg{
            name: "Test-cw20".to_string(),
            symbol: "ABT".to_string(),
            decimals: 6,
            initial_balances: vec![],
            mint: Some(MinterResponse{
                minter: chain.sender().to_string(),
                cap: None
            }),
            marketing: None
        }, None, None)?;

    Ok(())
}

/// Set up the test environment with the contract installed
fn setup() -> anyhow::Result<(AbstractAccount<Daemon>, Abstract<Daemon>, AppInterface<Daemon>)> {
    dotenv::dotenv().ok();
    let local_mnemonic = std::env::var("LOCAL_MNEMONIC").expect("LOCAL_MNEMONIC must be set");

    let runtime = Runtime::new()?;
    let daemon = Daemon::builder()
        .chain(LOCAL_OKP4)
        .mnemonic(local_mnemonic)
        .handle(runtime.handle())
        .build()
        .unwrap();

    let app = AppInterface::new(APP_ID, daemon.clone());

    let abstract_deployment = Abstract::deploy_on(daemon.clone(), daemon.sender().to_string())?;
    // let abstract_deployment = Abstract::load_from(daemon.clone())?;

    let account = abstract_deployment.
    account_factory.
    create_default_account(
        GovernanceDetails::Monarchy {
            monarch: daemon.sender().into_string(),
        },
    )?;

    abstract_deployment
        .version_control
        .claim_namespace(AccountId::local(1), "my-namespace".to_string())?;

    app.deploy(APP_VERSION.parse()?)?;

    account.install_app(app.clone(), &AppInstantiateMsg {}, None)?;

    Ok((account, abstract_deployment, app))
}

#[test]
fn successful_install() -> anyhow::Result<()> {
    let (_account, _abstr, app) = setup()?;

    let config = app.config()?;
    assert_eq!(config, ConfigResponse {});
    Ok(())
}

#[test]
fn do_some_dao_stuff() -> anyhow::Result<()> {
    Ok(())
}