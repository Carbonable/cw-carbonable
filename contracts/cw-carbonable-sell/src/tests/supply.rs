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
            pre_sell_mode: false,
            sell_mode: true,
            max_buy_at_once: 5,
        };

        let info = mock_info("owner_addr", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    #[test]
    fn update_supply_non_admin() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("test", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateSupply {
                reserved_supply: 2,
                market_supply: 2,
            },
        );

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::Unauthorized {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn update_supply_ok() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateSupply {
                reserved_supply: 2,
                market_supply: 2,
            },
        );

        assert!(res.is_ok());

        // query state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::DumpState {}).unwrap();
        let state: State = from_binary(&res).unwrap();
        assert_eq!(2, state.total_reserved_supply);
        assert_eq!(2, state.total_market_supply);
        assert_eq!(0, state.total_reserved_minted);
        assert_eq!(0, state.total_market_minted);
    }

    #[test]
    fn update_supply_twice() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::UpdateSupply {
                reserved_supply: 2,
                market_supply: 2,
            },
        );

        assert!(res.is_ok());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::DumpState {}).unwrap();
        let state: State = from_binary(&res).unwrap();
        assert_eq!(2, state.total_reserved_supply);
        assert_eq!(2, state.total_market_supply);
        assert_eq!(0, state.total_reserved_minted);
        assert_eq!(0, state.total_market_minted);

        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateSupply {
                reserved_supply: 0,
                market_supply: 0,
            },
        );

        assert!(res.is_ok());

        // query state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::DumpState {}).unwrap();
        let state: State = from_binary(&res).unwrap();
        assert_eq!(0, state.total_reserved_supply);
        assert_eq!(0, state.total_market_supply);
        assert_eq!(0, state.total_reserved_minted);
        assert_eq!(0, state.total_market_minted);
    }

    #[test]
    fn update_supply_airdrop() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateSupply {
                reserved_supply: 2,
                market_supply: 2,
            },
        );

        assert!(res.is_ok());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::DumpState {}).unwrap();
        let state: State = from_binary(&res).unwrap();
        assert_eq!(2, state.total_reserved_supply);
        assert_eq!(2, state.total_market_supply);
        assert_eq!(0, state.total_reserved_minted);
        assert_eq!(0, state.total_market_minted);

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::Airdrop {
                receivers: vec![String::from(
                    "cosmos1hdxjsex4frhtyzx68837fc7ssutp76kk4cyy4e",
                )],
            },
        );

        assert!(res.is_ok());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::DumpState {}).unwrap();
        let state: State = from_binary(&res).unwrap();
        assert_eq!(2, state.total_reserved_supply);
        assert_eq!(2, state.total_market_supply);
        assert_eq!(1, state.total_reserved_minted);
        assert_eq!(0, state.total_market_minted);

        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateSupply {
                reserved_supply: 0,
                market_supply: 0,
            },
        );

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::NotEnoughNftLeft {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn update_supply_buy() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateSupply {
                reserved_supply: 2,
                market_supply: 2,
            },
        );

        assert!(res.is_ok());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::DumpState {}).unwrap();
        let state: State = from_binary(&res).unwrap();
        assert_eq!(2, state.total_reserved_supply);
        assert_eq!(2, state.total_market_supply);
        assert_eq!(0, state.total_reserved_minted);
        assert_eq!(0, state.total_market_minted);

        let info = mock_info("owner_addr", &[coin(2_u128, String::from("ujuno"))]);
        let res = execute(deps.as_mut(), mock_env(), info.clone(), ExecuteMsg::Buy {});

        assert!(res.is_ok());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::DumpState {}).unwrap();
        let state: State = from_binary(&res).unwrap();
        assert_eq!(2, state.total_reserved_supply);
        assert_eq!(2, state.total_market_supply);
        assert_eq!(0, state.total_reserved_minted);
        assert_eq!(1, state.total_market_minted);

        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateSupply {
                reserved_supply: 0,
                market_supply: 0,
            },
        );

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::NotEnoughNftLeft {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn update_supply_buy_all_airdrop_all() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateSupply {
                reserved_supply: 2,
                market_supply: 2,
            },
        );

        assert!(res.is_ok());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::DumpState {}).unwrap();
        let state: State = from_binary(&res).unwrap();
        assert_eq!(2, state.total_reserved_supply);
        assert_eq!(2, state.total_market_supply);
        assert_eq!(0, state.total_reserved_minted);
        assert_eq!(0, state.total_market_minted);

        let info = mock_info("owner_addr", &[coin(2_u128, String::from("ujuno"))]);
        let res = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::Buy {});

        assert!(res.is_ok());

        let info = mock_info("owner_addr", &[coin(2_u128, String::from("ujuno"))]);
        let res = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::Buy {});

        assert!(res.is_ok());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::DumpState {}).unwrap();
        let state: State = from_binary(&res).unwrap();
        assert_eq!(2, state.total_reserved_supply);
        assert_eq!(2, state.total_market_supply);
        assert_eq!(0, state.total_reserved_minted);
        assert_eq!(2, state.total_market_minted);

        let info = mock_info("owner_addr", &[coin(2_u128, String::from("ujuno"))]);
        let res = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::Buy {});

        assert!(res.is_err());

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::NotEnoughNftLeft {} => {}
            _ => unreachable!(),
        }

        let info = mock_info("owner_addr", &[coin(2_u128, String::from("ujuno"))]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Airdrop {
                receivers: vec![
                    String::from("cosmos1hdxjsex4frhtyzx68837fc7ssutp76kk4cyy4e"),
                    String::from("cosmos1hdxjsex4frhtyzx68837fc7ssutp76kk4cyy4e"),
                ],
            },
        );

        assert!(res.is_ok());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::DumpState {}).unwrap();
        let state: State = from_binary(&res).unwrap();
        assert_eq!(2, state.total_reserved_supply);
        assert_eq!(2, state.total_market_supply);
        assert_eq!(2, state.total_reserved_minted);
        assert_eq!(2, state.total_market_minted);

        let info = mock_info("owner_addr", &[coin(2_u128, String::from("ujuno"))]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Airdrop {
                receivers: vec![String::from(
                    "cosmos1hdxjsex4frhtyzx68837fc7ssutp76kk4cyy4e",
                )],
            },
        );

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::NotEnoughNftLeft {} => {}
            _ => unreachable!(),
        }
    }
}
