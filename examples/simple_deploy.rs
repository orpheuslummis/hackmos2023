use cw_orch::{prelude::{Mock, CwOrchUpload, CwOrchInstantiate, CwEnv, DaemonBuilder}, tokio::runtime::Runtime, daemon::{ChainInfo, ChainKind, NetworkInfo}};
use cosmwasm_std::Addr;

use cw20_base::msg::InstantiateMsg;
use cw_plus_interface::cw20_base::Cw20Base;
use cw20::{msg::Cw20ExecuteMsgFns, MinterResponse};

use crate::

fn cw20<Chain: CwEnv>(chain: &Chain) -> anyhow::Result<()>{
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

    cw20.mint(80u128.into(),chain.sender().to_string())?;

    cw20.transfer(40u128.into(), chain.sender().to_string())?;

    Ok(())
}

fn main() {
    env_logger::init();
    dotenv::dotenv().unwrap();

    let sender = Addr::unchecked("sender");
    let mock = Mock::new(&sender);
    cw20(&mock).unwrap();

    let rt = Runtime::new().unwrap();
    let network = LOCAL_OKP4;
    let chain = DaemonBuilder::default()
        .handle(rt.handle())
        .chain(network)
        .build().unwrap();
    cw20(&chain).unwrap();


    // 
}

fn main() {}