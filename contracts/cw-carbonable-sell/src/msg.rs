use cosmwasm_std::{Addr, Coin};
use cw_carbonable_lib::Metadata;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub maintenance_mode: bool,
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

    Airdrop {
        receivers: Vec<String>,
    },
    Withdraw {
        wallet: Addr,
        coin: Vec<Coin>,
    },
    MaintenanceMode {
        enable: bool,
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
