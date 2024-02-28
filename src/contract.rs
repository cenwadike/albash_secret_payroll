use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::{
    execute,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{AdminStore, ContractStore, InvoiceStore},
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let admin = info.sender;
    AdminStore::save_admin_wallet(deps.storage, &admin)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SubmitInvoice {
            purpose,
            amount,
            admin_charge,
            customer_charge,
            payer,
            days,
            recurrent_time,
            token,
        } => execute::new_invoice(
            deps,
            env,
            info,
            purpose,
            amount,
            admin_charge,
            customer_charge,
            payer,
            days,
            recurrent_time,
            token,
        ),
        ExecuteMsg::AcceptInvoice { id } => execute::accept_invoice(deps, env, info, id),
        ExecuteMsg::CancelPayment { id } => execute::stop_contract(deps, env, info, id),
        ExecuteMsg::WithdrawPayment { id } => execute::withdraw_payment(deps, env, info, id),
        ExecuteMsg::AdminUpdateAmin { newAdmin } => execute::admin_change_admin(deps, env, info, newAdmin)
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::SingleInvoice { id, owner } => {
            to_binary(&InvoiceStore::load_invoice(deps.storage, &owner, id))
        }
        QueryMsg::NumberOfInvoice { owner } => {
            to_binary(&InvoiceStore::num_invoice(deps.storage, &owner))
        }
        QueryMsg::PaginatedInvoice {
            owner,
            page,
            page_size,
        } => to_binary(&InvoiceStore::paging_invoice_list(
            deps.storage,
            &owner,
            page,
            page_size,
        )?),
        QueryMsg::SingleContract { id, payer } => {
            to_binary(&ContractStore::load_contract(deps.storage, &payer, id))
        }
        QueryMsg::NumberOfContract { payer } => {
            to_binary(&ContractStore::num_contract(deps.storage, &payer))
        }
        QueryMsg::PaginatedContract {
            payer,
            page,
            page_size,
        } => to_binary(&ContractStore::paging_contract_list(
            deps.storage,
            &payer,
            page,
            page_size,
        )?),
        QueryMsg::AdmimWallet {  } => to_binary(
            &AdminStore::get_admin_wallet( deps.storage)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{from_binary, Coin, Uint128};
    use secret_toolkit::utils::types::Token;

    use crate::state::Contract;

    #[test]
    fn submit_invoice() {
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: "uscrt".to_string(),
            amount: Uint128::new(2),
        }]);

        let info = mock_info(
            "creator",
            &[Coin {
                denom: "uscrt".to_string(),
                amount: Uint128::new(2),
            }],
        );

        let init_msg = InstantiateMsg {};

        let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

        // anyone can submit invoice
        let info = mock_info(
            "anyone",
            &[Coin {
                denom: "uscrt".to_string(),
                amount: Uint128::new(2),
            }],
        );

        let exec_msg = ExecuteMsg::SubmitInvoice {
            purpose: "building".to_string(),
            //amount: Uint128::new(3),
            amount: 3,
            admin_charge: Uint128::new(3),
            customer_charge: Uint128::new(3),
            payer: "secret1py4ryg3atyz5cru2m64p0mtga5y09q5a26pa7n".to_string(),
            days: 2,
            recurrent_time: Some(2),
            token: Token::Native("uscrt".to_string()),
        };

        let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

        let info = mock_info(
            "anyone",
            &[Coin {
                denom: "uscrt".to_string(),
                amount: Uint128::new(2),
            }],
        );

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::NumberOfInvoice { owner: info.sender },
        )
        .unwrap();
        let value: u32 = from_binary(&res).unwrap();
        assert_eq!(1, value);
    }

    #[test]
    fn accept_invoice() {
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: "uscrt".to_string(),
            amount: Uint128::new(2),
        }]);

        let info = mock_info(
            "creator",
            &[Coin {
                denom: "uscrt".to_string(),
                amount: Uint128::new(2),
            }],
        );

        let init_msg = InstantiateMsg {};

        let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

        // anyone can submit invoice
        let info = mock_info(
            "anyone",
            &[Coin {
                denom: "uscrt".to_string(),
                amount: Uint128::new(2),
            }],
        );

        let exec_msg = ExecuteMsg::SubmitInvoice {
            purpose: "building".to_string(),
            //amount: Uint128::new(3),
            amount: 3,
            admin_charge: Uint128::new(3),
            customer_charge: Uint128::new(3),
            payer: "secret1py4ryg3atyz5cru2m64p0mtga5y09q5a26pa7n".to_string(),
            days: 2,
            recurrent_time: Some(2),
            token: Token::Native("uscrt".to_string()),
        };

        let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

        let info = mock_info(
            "anyone",
            &[Coin {
                denom: "uscrt".to_string(),
                amount: Uint128::new(2),
            }],
        );

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::NumberOfInvoice { owner: info.sender },
        )
        .unwrap();
        let value: u32 = from_binary(&res).unwrap();
        assert_eq!(1, value);

        let info = mock_info(
            "secret1py4ryg3atyz5cru2m64p0mtga5y09q5a26pa7n",
            &[Coin {
                denom: "uscrt".to_string(),
                amount: Uint128::new(6),
            }],
        );

        let exec_msg = ExecuteMsg::AcceptInvoice { id: 1 };

        let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

        let info = mock_info(
            "secret1py4ryg3atyz5cru2m64p0mtga5y09q5a26pa7n",
            &[Coin {
                denom: "uscrt".to_string(),
                amount: Uint128::new(0),
            }],
        );

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::SingleContract {
                id: 1,
                payer: info.sender,
            },
        )
        .unwrap();
        let value: Contract = from_binary(&res).unwrap();
        assert_eq!(true, value.contract_accepted);
        assert_eq!("started".to_string(), value.contract_process);
    }

    #[test]
    fn cancel_payment() {
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: "uscrt".to_string(),
            amount: Uint128::new(2),
        }]);

        let info = mock_info(
            "creator",
            &[Coin {
                denom: "uscrt".to_string(),
                amount: Uint128::new(2),
            }],
        );

        let init_msg = InstantiateMsg {};

        let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

        // anyone can submit invoice
        let info = mock_info(
            "anyone",
            &[Coin {
                denom: "uscrt".to_string(),
                amount: Uint128::new(2),
            }],
        );

        let exec_msg = ExecuteMsg::SubmitInvoice {
            purpose: "building".to_string(),
            //amount: Uint128::new(3),
            amount: 3,
            admin_charge: Uint128::new(3),
            customer_charge: Uint128::new(3),
            payer: "secret1py4ryg3atyz5cru2m64p0mtga5y09q5a26pa7n".to_string(),
            days: 2,
            recurrent_time: Some(2),
            token: Token::Native("uscrt".to_string()),
        };

        let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

        let info = mock_info(
            "anyone",
            &[Coin {
                denom: "uscrt".to_string(),
                amount: Uint128::new(2),
            }],
        );

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::NumberOfInvoice { owner: info.sender },
        )
        .unwrap();
        let value: u32 = from_binary(&res).unwrap();
        assert_eq!(1, value);

        let info = mock_info(
            "secret1py4ryg3atyz5cru2m64p0mtga5y09q5a26pa7n",
            &[Coin {
                denom: "uscrt".to_string(),
                amount: Uint128::new(6),
            }],
        );

        let exec_msg = ExecuteMsg::AcceptInvoice { id: 1 };

        let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

        let info = mock_info(
            "secret1py4ryg3atyz5cru2m64p0mtga5y09q5a26pa7n",
            &[Coin {
                denom: "uscrt".to_string(),
                amount: Uint128::new(0),
            }],
        );

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::SingleContract {
                id: 1,
                payer: info.sender,
            },
        )
        .unwrap();
        let value: Contract = from_binary(&res).unwrap();
        assert_eq!(true, value.contract_accepted);
        assert_eq!("started".to_string(), value.contract_process);

        let info = mock_info(
            "secret1py4ryg3atyz5cru2m64p0mtga5y09q5a26pa7n",
            &[Coin {
                denom: "uscrt".to_string(),
                amount: Uint128::new(6),
            }],
        );

        let exec_msg = ExecuteMsg::CancelPayment { id: 1 };

        let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

        let info = mock_info(
            "secret1py4ryg3atyz5cru2m64p0mtga5y09q5a26pa7n",
            &[Coin {
                denom: "uscrt".to_string(),
                amount: Uint128::new(0),
            }],
        );

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::SingleContract {
                id: 1,
                payer: info.sender,
            },
        )
        .unwrap();
        let value: Contract = from_binary(&res).unwrap();
        assert_eq!(0, value.account_balance);
        assert_eq!("stop".to_string(), value.contract_process);
        assert_eq!(Uint128::new(0), value.invoice.amount);
    }
}
