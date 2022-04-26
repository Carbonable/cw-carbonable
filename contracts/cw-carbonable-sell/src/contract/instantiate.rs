use crate::contract::{CONTRACT_NAME, CONTRACT_VERSION};
use crate::msg::InitMsg;
use crate::state::{
    State, ADMIN_WALLETS, NFT_CONTRACT, OWNER_WALLET, PRE_SELL_MODE, SELL_MODE, STATE,
};
use crate::ContractError;
use cosmwasm_std::{coin, Addr, DepsMut, MessageInfo, Response};
use cw2::set_contract_version;
use cw_carbonable_lib::Metadata;
use std::collections::HashSet;

pub fn _instantiate(
    deps: DepsMut,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    // contract owner
    OWNER_WALLET.save(deps.storage, &info.sender)?;

    // nft contract
    NFT_CONTRACT.save(deps.storage, &Addr::unchecked("unset"))?;

    // empty list of admins
    ADMIN_WALLETS.save(deps.storage, &HashSet::new())?;

    // set sales mode
    SELL_MODE.save(deps.storage, &msg.sell_mode)?;
    PRE_SELL_MODE.save(deps.storage, &msg.pre_sell_mode)?;

    let state = State {
        total_market_supply: 0,
        total_reserved_supply: 0,
        total_market_minted: 0,
        total_reserved_minted: 0,
        max_buy_at_once: msg.max_buy_at_once,
        last_token_id: 0,
        sell_price: coin(0u128, String::from("ujuno")),
        metadata: Metadata {
            image: None,
            image_data: None,
            external_url: None,
            description: None,
            name: None,
            attributes: None,
            background_color: None,
            animation_url: None,
            youtube_url: None,
        },
    };
    STATE.save(deps.storage, &state)?;

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}
