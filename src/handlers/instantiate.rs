use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::contract::{App, AppResult};
use crate::msg::AppInstantiateMsg;
use crate::state::{Config, State, CONFIG, STATE};

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _app: App,
    msg: AppInstantiateMsg,
) -> AppResult {
    let config: Config = Config {};
    CONFIG.save(deps.storage, &config)?;

    // instantiate with msg and info
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    STATE.save(deps.storage, &state)?;

    // Example instantiation with atttributes
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}
