use crate::msg::ExecuteMsg;
use crate::state::{State, ADMIN_WALLETS, MAINTENANCE_MODE, NFT_CONTRACT, OWNER_WALLET, STATE};
use crate::ContractError;
use cosmwasm_std::{
    has_coins, to_binary, Addr, BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response,
    Uint128, WasmMsg,
};
use cw_carbonable_lib::{Extension, Metadata};
use std::collections::HashSet;

pub fn _execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Buy {} => try_buy(deps, info),
        ExecuteMsg::MultiBuy { quantity } => try_multi_buy(deps, info, quantity),
        ExecuteMsg::Airdrop { receivers } => try_airdrop(deps, info, receivers),
        ExecuteMsg::Withdraw { wallet, coin } => try_withdraw(deps, info, wallet, coin),
        ExecuteMsg::MaintenanceMode { enable } => switch_mainteanance_mode(deps, info, enable),
        ExecuteMsg::UpdatePrice { price } => update_price(deps, info, price),
        ExecuteMsg::UpdateSupply {
            reserved_supply,
            market_supply,
        } => update_supply(deps, info, reserved_supply, market_supply),
        ExecuteMsg::UpdateMetadata { metadata } => update_metadata(deps, info, metadata),
        ExecuteMsg::UpdateNftContract { address } => update_nft_contract(deps, info, address),
        ExecuteMsg::RemoveAdmin { address } => remove_admin(deps, info, address),
        ExecuteMsg::AddAdmin { address } => add_admin(deps, info, address),
    }
}

pub fn try_buy(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    // load state
    let mut state = STATE.load(deps.storage)?;

    // Is contract in maintenance ?
    is_in_maintenance(&deps)?;

    state.total_market_minted += 1;
    // Is some NFT available ?
    is_market_nft_available(&state)?;

    // Does the buy has enough coins ?
    if !has_coins(info.funds.as_slice(), &state.sell_price) {
        return Err(ContractError::NotEnoughMoneyForNft {});
    }

    // Bump last_token_id ?
    state.last_token_id += 1;

    let response = Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: NFT_CONTRACT.load(deps.storage)?.to_string(),
        msg: to_binary(&mint_helper(
            state.last_token_id,
            info.sender.to_string(),
            state.clone(),
        ))?,
        funds: vec![],
    }));

    // Update contract state
    STATE.save(deps.storage, &state)?;

    Ok(response.add_attribute("method", "try_buy"))
}

pub fn try_multi_buy(
    deps: DepsMut,
    info: MessageInfo,
    quantity: u32,
) -> Result<Response, ContractError> {
    // load state
    let mut state = STATE.load(deps.storage)?;

    // Is contract in maintenance ?
    is_in_maintenance(&deps)?;

    // Check quantity validity
    is_multi_buy_quantity_too_big(quantity, &state)?;

    state.total_market_minted += quantity;
    // Is some NFT available ?
    is_market_nft_available(&state)?;

    // Does the buy has enough coins ?
    let mut sell_price = state.sell_price.clone();
    sell_price.amount *= Uint128::from(quantity);
    if !has_coins(info.funds.as_slice(), &sell_price) {
        return Err(ContractError::NotEnoughMoneyForNft {});
    }

    let mut response = Response::new();
    for _ in 0..quantity {
        // Bump last_token_id ?
        state.last_token_id += 1;

        response = response.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: NFT_CONTRACT.load(deps.storage)?.to_string(),
            msg: to_binary(&mint_helper(
                state.last_token_id,
                info.sender.to_string(),
                state.clone(),
            ))?,
            funds: vec![],
        }));
    }

    // Update contract state
    STATE.save(deps.storage, &state)?;

    Ok(response.add_attribute("method", "try_multi_buy"))
}

pub fn update_nft_contract(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    // Is contract in maintenance ?
    is_in_maintenance(&deps)?;

    // Is admin or owner wallet ?
    is_admin_or_owner(&deps, info)?;

    // check validity of new admin address
    let contract_address = deps.api.addr_validate(address.as_str());
    if contract_address.is_err() {
        return Err(ContractError::InvalidAddress { address });
    }

    NFT_CONTRACT.save(deps.storage, &contract_address.unwrap())?;

    Ok(Response::new().add_attribute("method", "update_nft_contract"))
}

pub fn update_metadata(
    deps: DepsMut,
    info: MessageInfo,
    metadata: Metadata,
) -> Result<Response, ContractError> {
    // load state
    let mut state = STATE.load(deps.storage)?;

    // Is contract in maintenance ?
    is_in_maintenance(&deps)?;

    // Is admin or owner wallet ?
    is_admin_or_owner(&deps, info)?;

    // check validity of new admin address
    state.metadata = metadata;

    // Update contract state
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("method", "update_metadata"))
}

pub fn try_airdrop(
    deps: DepsMut,
    info: MessageInfo,
    receiver: Vec<String>,
) -> Result<Response, ContractError> {
    // load state
    let mut state = STATE.load(deps.storage)?;

    // Is contract in maintenance ?
    is_in_maintenance(&deps)?;

    // Is admin or owner wallet ?
    is_admin_or_owner(&deps, info)?;

    state.total_reserved_minted += receiver.len() as u32;
    // Is some NFT available ?
    is_reserved_nft_available(&state)?;

    // Check validity of drop receivers addresses
    is_addresses_valid(&deps, &receiver)?;

    // Mint
    let mut response = Response::new();
    for recv in receiver {
        state.last_token_id += 1;

        response = response.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: NFT_CONTRACT.load(deps.storage)?.to_string(),
            msg: to_binary(&mint_helper(
                state.last_token_id,
                recv.to_string(),
                state.clone(),
            ))?,
            funds: vec![],
        }))
    }

    // Update contract state
    STATE.save(deps.storage, &state)?;

    Ok(response.add_attribute("method", "try_airdrop"))
}

pub fn try_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    wallet: Addr,
    coin: Vec<Coin>,
) -> Result<Response, ContractError> {
    // Is contract in maintenance ?
    is_in_maintenance(&deps)?;

    // Is admin or owner wallet ?
    is_admin_or_owner(&deps, info)?;

    let send_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: wallet.to_string(),
        amount: coin,
    });
    Ok(Response::new()
        .add_message(send_msg)
        .add_attribute("method", "try_withdraw"))
}

pub fn switch_mainteanance_mode(
    deps: DepsMut,
    info: MessageInfo,
    enable: bool,
) -> Result<Response, ContractError> {
    // Is admin or owner wallet ?
    is_admin_or_owner(&deps, info)?;

    MAINTENANCE_MODE.save(deps.storage, &enable)?;

    Ok(Response::new()
        .add_attribute("method", "switch_mainteanance_mode")
        .add_attribute("status", enable.to_string()))
}

pub fn update_price(
    deps: DepsMut,
    info: MessageInfo,
    price: Coin,
) -> Result<Response, ContractError> {
    // load state
    let mut state = STATE.load(deps.storage)?;

    // Is contract in maintenance ?
    is_in_maintenance(&deps)?;

    // Is admin or owner wallet ?
    is_admin_or_owner(&deps, info)?;

    state.sell_price = price;

    // Update contract state
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("method", "update_price"))
}

pub fn update_supply(
    deps: DepsMut,
    info: MessageInfo,
    reserved_supply: u32,
    market_supply: u32,
) -> Result<Response, ContractError> {
    // load state
    let mut state = STATE.load(deps.storage)?;

    // Is contract in maintenance ?
    is_in_maintenance(&deps)?;

    // Is admin or owner wallet ?
    is_admin_or_owner(&deps, info)?;

    state.total_market_supply = market_supply;
    state.total_reserved_supply = reserved_supply;

    // Check if new supply is ok
    is_market_nft_available(&state)?;
    is_reserved_nft_available(&state)?;

    // Update contract state
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("method", "update_price"))
}

pub fn remove_admin(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    // Is contract in maintenance ?
    is_in_maintenance(&deps)?;

    // Security : owner has all rights
    is_admin_or_owner(&deps, info)?;

    // check validity of new admin address
    let admin_to_remove = deps.api.addr_validate(address.as_str());
    if admin_to_remove.is_err() {
        return Err(ContractError::InvalidAddress { address });
    }

    // check if new admin
    if !ADMIN_WALLETS
        .load(deps.storage)?
        .contains(admin_to_remove.as_ref().unwrap())
    {
        return Err(ContractError::AddressNotFound { address });
    }

    ADMIN_WALLETS.update(
        deps.storage,
        |mut wallets| -> Result<HashSet<Addr>, ContractError> {
            wallets.remove(&admin_to_remove.unwrap());
            Ok(wallets)
        },
    )?;

    let response = Response::new();
    Ok(response.add_attribute("method", "remove_admin"))
}

pub fn add_admin(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    // Is contract in maintenance ?
    is_in_maintenance(&deps)?;

    // Security : owner has all rights
    is_admin_or_owner(&deps, info)?;

    // check validity of new admin address
    let admin_to_add = deps.api.addr_validate(address.as_str());
    if admin_to_add.is_err() {
        return Err(ContractError::InvalidAddress { address });
    }

    // check if new admin
    if ADMIN_WALLETS
        .load(deps.storage)?
        .contains(admin_to_add.as_ref().unwrap())
    {
        return Err(ContractError::AddressAlreadyRegistered { address });
    }

    ADMIN_WALLETS.update(
        deps.storage,
        |mut wallets| -> Result<HashSet<Addr>, ContractError> {
            wallets.insert(admin_to_add.unwrap());
            Ok(wallets)
        },
    )?;

    let response = Response::new();
    Ok(response.add_attribute("method", "add_admin"))
}

fn mint_helper(nft_id: u32, addr: String, state: State) -> cw_carbonable_lib::ExecuteMsg {
    cw_carbonable_lib::ExecuteMsg::Mint(cw721_base::MintMsg::<Extension> {
        token_id: nft_id.to_string(),

        owner: addr,
        token_uri: None,
        extension: Extension::from(state.metadata),
    })
}

pub fn is_in_maintenance(deps: &DepsMut) -> Result<(), ContractError> {
    if MAINTENANCE_MODE.load(deps.storage)? {
        return Err(ContractError::InMaintenance {});
    }

    Ok(())
}

pub fn is_market_nft_available(state: &State) -> Result<(), ContractError> {
    if state.total_market_minted > state.total_market_supply {
        return Err(ContractError::NotEnoughNftLeft {});
    }

    Ok(())
}

pub fn is_reserved_nft_available(state: &State) -> Result<(), ContractError> {
    if state.total_reserved_minted > state.total_reserved_supply {
        return Err(ContractError::NotEnoughNftLeft {});
    }

    Ok(())
}

pub fn is_multi_buy_quantity_too_big(quantity: u32, state: &State) -> Result<(), ContractError> {
    if quantity > state.max_buy_at_once {
        return Err(ContractError::MultiBuyQuantityTooHigh {});
    }

    Ok(())
}

pub fn is_admin_or_owner(deps: &DepsMut, info: MessageInfo) -> Result<(), ContractError> {
    if OWNER_WALLET.load(deps.storage)? != info.sender {
        // Security : non-owner check if send is admin
        if !ADMIN_WALLETS.load(deps.storage)?.contains(&info.sender) {
            return Err(ContractError::Unauthorized {});
        }
    }

    Ok(())
}

pub fn is_addresses_valid(deps: &DepsMut, receiver: &[String]) -> Result<(), ContractError> {
    // Check validity of drop receivers addresses
    for client in receiver {
        match deps.api.addr_validate(client.as_str()) {
            Ok(_) => {}
            Err(_) => {
                return Err(ContractError::InvalidAddress {
                    address: client.clone(),
                });
            }
        }
    }

    Ok(())
}
