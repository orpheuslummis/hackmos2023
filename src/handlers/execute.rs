use abstract_sdk::features::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};

use crate::contract::{App, AppResult};

use crate::msg::AppExecuteMsg;
use crate::state::{CONFIG, STATE as STATE_2};

pub fn execute_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    app: App,
    msg: AppExecuteMsg,
) -> AppResult {
    match msg {
        AppExecuteMsg::UpdateConfig {} => update_config(deps, info, app),
        AppExecuteMsg::Increment {} => increment(deps, info, app),
        AppExecuteMsg::Reset { count } => reset(deps, info, app, count),
    }
}

// HACKMOS
// rust proto types

/// Update the configuration of the app
fn update_config(deps: DepsMut, msg_info: MessageInfo, app: App) -> AppResult {
    // Only the admin should be able to call this
    app.admin.assert_admin(deps.as_ref(), &msg_info.sender)?;
    let mut _config = CONFIG.load(deps.storage)?;
    // deps.querier.query(req);
    Ok(app.tag_response(Response::default(), "update_config"))
}

fn increment(deps: DepsMut, _msg_info: MessageInfo, app: App) -> AppResult {
    let _ = STATE_2.update(deps.storage, |mut state| -> StdResult<_> {
        state.count += 1;
        Ok(state)
    });

    Ok(app.tag_response(Response::default(), "increment"))
}

fn reset(deps: DepsMut, _msg_info: MessageInfo, app: App, count: i32) -> AppResult {
    let _ = STATE_2.update(deps.storage, |mut state| -> StdResult<_> {
        state.count = count;
        Ok(state)
    });

    Ok(app.tag_response(Response::default(), "reset"))
}

// pub fn increment(deps: DepsMut) -> AppResult {
//     STATE_2.update(deps.storage, |mut state: State| -> Result<State, _> {
//         state.count += 1;
//         Ok(state)
//     })?;

//     Ok(Response::new().add_attribute("action", "increment"))
// }
