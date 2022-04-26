use cosmwasm_std::{Addr, Coin};
use cw_carbonable_lib::Metadata;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub sell_mode: bool,
    pub pre_sell_mode: bool,
    pub max_buy_at_once: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WhiteListEntry {
    pub address: String,
    pub nb_slots: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateSupply {
        reserved_supply: u32,
        market_supply: u32,
    },
    UpdatePrice {
        price: Coin,
    },
    UpdateNftContract {
        address: String,
    },
    UpdateMetadata {
        metadata: Metadata,
    },
    Buy {},
    MultiBuy {
        quantity: u32,
    },
    Airdrop {
        receivers: Vec<String>,
    },
    Withdraw {
        wallet: Addr,
        coin: Vec<Coin>,
    },
    PreSellMode {
        enable: bool,
    },
    SellMode {
        enable: bool,
    },
    AddToWhitelist {
        entries: Vec<WhiteListEntry>,
    },

    /// Roles mgmt
    RemoveAdmin {
        address: String,
    },
    AddAdmin {
        address: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // Return the contract state
    DumpState {},
}
