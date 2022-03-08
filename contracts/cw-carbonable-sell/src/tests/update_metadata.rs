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
    use cosmwasm_std::{coins, from_binary, OwnedDeps};
    use cw_carbonable_lib::Metadata;

    fn helper_instantiate(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier>) {
        let msg = InitMsg {
            maintenance_mode: false,
        };

        let info = mock_info("owner_addr", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    #[test]
    fn update_metadata_unauthorized() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("test", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateMetadata {
                metadata: Metadata {
                    name: Some(String::from("Carbonable")),
                    youtube_url: Some(String::from("www.youtube.com")),
                    ..Default::default()
                },
            },
        );

        assert!(res.is_err());
        match res.err().unwrap() {
            ContractError::Unauthorized {} => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn update_metadata_ok() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateMetadata {
                metadata: Metadata {
                    name: Some(String::from("Carbonable")),
                    youtube_url: Some(String::from("www.youtube.com")),
                    ..Default::default()
                },
            },
        );

        assert!(res.is_ok());

        // query state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::DumpState {}).unwrap();
        let state: State = from_binary(&res).unwrap();
        assert_eq!(state.metadata.name.unwrap(), "Carbonable".to_string());
        assert_eq!(
            state.metadata.youtube_url.unwrap(),
            "www.youtube.com".to_string()
        );
    }

    #[test]
    fn update_metadata_twice() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        helper_instantiate(&mut deps);

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateMetadata {
                metadata: Metadata {
                    name: Some(String::from("Carbonable")),
                    youtube_url: Some(String::from("www.youtube.com")),
                    ..Default::default()
                },
            },
        );

        assert!(res.is_ok());

        // query state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::DumpState {}).unwrap();
        let state: State = from_binary(&res).unwrap();
        assert_eq!(state.metadata.name.unwrap(), "Carbonable".to_string());
        assert_eq!(
            state.metadata.youtube_url.unwrap(),
            "www.youtube.com".to_string()
        );

        let info = mock_info("owner_addr", &[]);
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdateMetadata {
                metadata: Metadata {
                    name: Some(String::from("Carbonable 2")),
                    youtube_url: Some(String::from("www.dailymotion.com")),
                    ..Default::default()
                },
            },
        );

        assert!(res.is_ok());

        // query state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::DumpState {}).unwrap();
        let state: State = from_binary(&res).unwrap();
        assert_eq!(state.metadata.name.unwrap(), "Carbonable 2".to_string());
        assert_eq!(
            state.metadata.youtube_url.unwrap(),
            "www.dailymotion.com".to_string()
        );
    }
}
