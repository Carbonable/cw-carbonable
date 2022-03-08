#[cfg(test)]
mod cw_carbonable {
    use crate::contract::{execute, instantiate};
    use crate::msg::ExecuteMsg;
    use crate::msg::InitMsg;
    use crate::ContractError;
    use cosmwasm_std::testing::{
        mock_dependencies_with_balance, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{coin, coins, Addr, OwnedDeps};

    fn helper_instantiate(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier>) {
        let msg = InitMsg {
            maintenance_mode: true,
        };

        let info = mock_info("owner_addr", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    #[test]
    fn maintenance_update_supply() {
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
            ContractError::InMaintenance {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn maintenance_buy() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("test", &[]);
        let res = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::Buy {});

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::InMaintenance {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn maintenance_airdrop() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("test", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Airdrop { receivers: vec![] },
        );

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::InMaintenance {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn maintenance_withdraw() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("test", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Withdraw {
                wallet: Addr::unchecked("test"),
                coin: vec![coin(1u128, String::from("ujuno"))],
            },
        );

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::InMaintenance {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn maintenance_update_price() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("test", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdatePrice {
                price: coin(1u128, String::from("ujuno")),
            },
        );

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::InMaintenance {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn maintenance_add_admin() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("test", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::AddAdmin {
                address: "".to_string(),
            },
        );

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::InMaintenance {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn maintenance_remove_admin() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("test", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::RemoveAdmin {
                address: "".to_string(),
            },
        );

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::InMaintenance {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn maintenance_disable_unauthorized() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("test", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::MaintenanceMode { enable: true },
        );

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::Unauthorized {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn maintenance_disable() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::MaintenanceMode { enable: true },
        );

        assert!(res.is_ok());
    }
}
