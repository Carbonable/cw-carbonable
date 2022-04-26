#[cfg(test)]
mod cw_carbonable {
    use crate::contract::{execute, instantiate};
    use crate::msg::InitMsg;
    use crate::msg::{ExecuteMsg, WhiteListEntry};
    use crate::ContractError;
    use cosmwasm_std::testing::{
        mock_dependencies_with_balance, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{coin, coins, OwnedDeps};

    fn helper_instantiate(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier>) {
        let msg = InitMsg {
            pre_sell_mode: false,
            sell_mode: false,
            max_buy_at_once: 5,
        };

        let info = mock_info("owner_addr", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    #[test]
    fn sell_off() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("test", &[]);
        let res = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::Buy {});

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::SellClose {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn presale_on_not_in_whitelist() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::PreSellMode { enable: true },
        );
        assert!(res.is_ok());

        let info = mock_info("test", &[]);
        let res = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::Buy {});

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::AddressNotWhitelisted {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn presale_ok() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::AddToWhitelist {
                entries: vec![WhiteListEntry {
                    address: String::from("test"),
                    nb_slots: 2,
                }],
            },
        );
        assert!(res.is_ok());

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateSupply {
                reserved_supply: 0,
                market_supply: 10
            },
        );
        assert!(res.is_ok());

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdatePrice {
                price: {
                    coin(4, String::from("juno"))
                }
            },
        );
        assert!(res.is_ok());

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::PreSellMode { enable: true },
        );
        assert!(res.is_ok());

        let info = mock_info("test", &[coin(8, String::from("juno"))]);
        let res = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::MultiBuy { quantity: 2});

        println!("{:#?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn presale_too_many() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::AddToWhitelist {
                entries: vec![WhiteListEntry {
                    address: String::from("test"),
                    nb_slots: 1,
                }],
            },
        );
        assert!(res.is_ok());

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateSupply {
                reserved_supply: 0,
                market_supply: 10
            },
        );
        assert!(res.is_ok());

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdatePrice {
                price: {
                    coin(4, String::from("juno"))
                }
            },
        );
        assert!(res.is_ok());

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::PreSellMode { enable: true },
        );
        assert!(res.is_ok());

        let info = mock_info("test", &[coin(8, String::from("juno"))]);
        let res = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::MultiBuy { quantity: 2});

        println!("{:#?}", res);
        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::NoSlotAvailableLeft {} => {}
            _ => unreachable!(),
        }
    }
}
