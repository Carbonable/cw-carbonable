use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use cosmwasm_std::{Addr, Coin};
use cw_carbonable_lib::Metadata;
use cw_storage_plus::{Item, Map};

pub type TokenID = String;

pub static CONFIG_KEY: &[u8] = b"config";

/// Supply State
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub total_market_supply: u32,
    pub total_reserved_supply: u32,
    pub total_market_minted: u32,
    pub total_reserved_minted: u32,
    pub max_buy_at_once: u32,
    pub last_token_id: u32,
    pub sell_price: Coin,
    pub metadata: Metadata,
}

pub const STATE: Item<State> = Item::new("state");

/// Address of owner wallet
///
/// equivalent of a root user for the contract
pub const OWNER_WALLET: Item<Addr> = Item::new("owner_wallet");

/// Address of admins wallet
///
/// These user have these differents role
/// * add/remove admin wallets
/// * add/remove nft id in nft map
/// * withdraw from smartcontract wallet
pub const ADMIN_WALLETS: Item<HashSet<Addr>> = Item::new("admin_wallets");

/// Address of carbonable NFT contract
///
/// These user have this role
/// * add nft_ids to track
pub const NFT_CONTRACT: Item<Addr> = Item::new("nft_contract");

/// Sale is activated
pub const SELL_MODE: Item<bool> = Item::new("sell_mode");

/// Pre Sale activated
pub const PRE_SELL_MODE: Item<bool> = Item::new("pre_sell_mode");

/// WhiteList map
pub const WHITELIST: Map<Addr, u32> = Map::new("whitelist");
