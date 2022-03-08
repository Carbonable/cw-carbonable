use crate::msg::QueryMsg;
use crate::state::{State, STATE};
use cosmwasm_std::{to_binary, Binary, Deps, StdResult};

pub fn _query(deps: Deps, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::DumpState {} => to_binary(&dump_state(deps)?),
    }
}

pub fn dump_state(deps: Deps) -> StdResult<State> {
    let state = STATE.load(deps.storage)?;

    Ok(state)
}
