#[cfg(test)]
mod cw_carbonable {
    use crate::contract::{execute, instantiate};
    use crate::msg::ExecuteMsg;
    use crate::msg::InitMsg;
    use crate::ContractError;
    use cosmwasm_std::testing::{
        mock_dependencies_with_balance, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{coins, OwnedDeps};

    fn helper_instantiate(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier>) {
        let msg = InitMsg {
            sell_mode: true,
            pre_sell_mode: false,
            max_buy_at_once: 5,
        };

        let info = mock_info("owner_addr", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let _ = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::AddAdmin {
                address: String::from("admin1_addr"),
            },
        );

        let _ = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::AddAdmin {
                address: String::from("admin2_addr"),
            },
        );
    }

    #[test]
    fn unauthorized_add_admin() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("test", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::AddAdmin {
                address: String::from("invalid_admin"),
            },
        );

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::Unauthorized {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn owner_add_admin() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::AddAdmin {
                address: String::from("new_admin"),
            },
        );

        assert!(res.is_ok());
    }

    #[test]
    fn admin_add_admin() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("admin1_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::AddAdmin {
                address: String::from("new_admin"),
            },
        );

        assert!(res.is_ok());
    }

    #[test]
    fn add_already_existing_wallet() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("admin1_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::AddAdmin {
                address: String::from("admin2_addr"),
            },
        );

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::AddressAlreadyRegistered { address } => {
                assert_eq!(address.as_str(), "admin2_addr")
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn unauthorized_remove_admin() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("test", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::RemoveAdmin {
                address: String::from("admin2_addr"),
            },
        );

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::Unauthorized {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn owner_remove_admin() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::RemoveAdmin {
                address: String::from("admin2_addr"),
            },
        );

        assert!(res.is_ok());
    }

    #[test]
    fn admin_remove_admin() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("admin1_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::RemoveAdmin {
                address: String::from("admin2_addr"),
            },
        );

        assert!(res.is_ok());
    }

    #[test]
    fn remove_unexisting_wallet() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("admin1_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::RemoveAdmin {
                address: String::from("yolo"),
            },
        );

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::AddressNotFound { address } => {
                assert_eq!(address.as_str(), "yolo")
            }
            _ => unreachable!(),
        }
    }
}
