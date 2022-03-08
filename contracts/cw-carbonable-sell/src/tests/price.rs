#[cfg(test)]
mod cw_carbonable {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::InitMsg;
    use crate::msg::{ExecuteMsg, QueryMsg};
    use crate::state::State;
    use crate::ContractError;
    use cosmwasm_std::testing::{
        mock_dependencies_with_balance, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{coin, coins, from_binary, OwnedDeps};

    fn helper_instantiate(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier>) {
        let msg = InitMsg {
            maintenance_mode: false,
        };

        let info = mock_info("owner_addr", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::UpdatePrice {
                price: coin(20u128, String::from("ujuno")),
            },
        );

        assert!(res.is_ok());

        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateSupply {
                reserved_supply: 0,
                market_supply: 1,
            },
        );

        assert!(res.is_ok());
    }

    #[test]
    fn update_price_non_admin() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("cosmos1hdxjsex4frhtyzx68837fc7ssutp76kk4cyy4e", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdatePrice {
                price: coin(20u128, String::from("ujuno")),
            },
        );

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::Unauthorized {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn update_price_ok() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdatePrice {
                price: coin(20u128, String::from("ujuno")),
            },
        );

        assert!(res.is_ok());
    }

    #[test]
    fn buy_not_enough_money() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info(
            "cosmos1hdxjsex4frhtyzx68837fc7ssutp76kk4cyy4e",
            &[coin(18u128, String::from("ujuno"))],
        );
        let res = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::Buy {});

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::NotEnoughMoneyForNft {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn buy_bad_currency() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info(
            "cosmos1hdxjsex4frhtyzx68837fc7ssutp76kk4cyy4e",
            &[coin(18u128, String::from("uatom"))],
        );
        let res = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::Buy {});

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::NotEnoughMoneyForNft {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn buy_ok() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info(
            "cosmos1hdxjsex4frhtyzx68837fc7ssutp76kk4cyy4e",
            &[coin(20u128, String::from("ujuno"))],
        );
        let res = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::Buy {});

        assert!(res.is_ok());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::DumpState {}).unwrap();
        let state: State = from_binary(&res).unwrap();
        assert_eq!(0, state.total_reserved_supply);
        assert_eq!(1, state.total_market_supply);
        assert_eq!(0, state.total_reserved_minted);
        assert_eq!(1, state.total_market_minted);
    }
}
