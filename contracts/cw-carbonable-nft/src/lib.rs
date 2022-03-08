use cosmwasm_std::Empty;
pub use cw721_base::{ContractError, InstantiateMsg, MintMsg, MinterResponse, QueryMsg};
use cw_carbonable_lib::Extension;

pub type CarbonableMetadataContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-carbonable-nft";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use cw2::set_contract_version;
    use cw_carbonable_lib::ExecuteMsg;

    // This is a simple type to let us handle empty extensions

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        let res =
            CarbonableMetadataContract::default().instantiate(deps.branch(), env, info, msg)?;
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(res)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        CarbonableMetadataContract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        CarbonableMetadataContract::default().query(deps, env, msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw721::Cw721Query;
    use cw_carbonable_lib::{ExecuteMsg, Metadata};

    const CREATOR: &str = "creator";

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = CarbonableMetadataContract::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "SpaceShips".to_string(),
            symbol: "SPACE".to_string(),
            minter: CREATOR.to_string(),
        };
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let token_id = "Enterprise";
        let mint_msg = MintMsg {
            token_id: token_id.to_string(),
            owner: "john".to_string(),
            token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
            extension: Some(Metadata {
                description: Some("Spaceship with Warp Drive".into()),
                name: Some("Starship USS Enterprise".to_string()),
                ..Metadata::default()
            }),
        };
        let exec_msg = ExecuteMsg::Mint(mint_msg.clone());
        contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        let res = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();
        assert_eq!(res.token_uri, mint_msg.token_uri);
        assert_eq!(res.extension, mint_msg.extension);
    }
}
