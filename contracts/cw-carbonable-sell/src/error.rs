use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Sell close")]
    SellClose {},

    #[error("Address not whitelisted")]
    AddressNotWhitelisted {},

    #[error("No slot available left")]
    NoSlotAvailableLeft {},

    #[error("Not enought nft left")]
    NotEnoughNftLeft {},

    #[error("Multibuy quantity too high>")]
    MultiBuyQuantityTooHigh {},

    #[error("Invalid address {address:?}")]
    InvalidAddress { address: String },

    #[error("Address not found {address:?}")]
    AddressNotFound { address: String },

    #[error("Address already registred {address:?}")]
    AddressAlreadyRegistered { address: String },

    #[error("Not enough Money")]
    NotEnoughMoneyForNft {}, // Add any other custom errors you like here.
                             // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
