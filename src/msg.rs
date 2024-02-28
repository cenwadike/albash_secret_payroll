#![allow(clippy::field_reassign_with_default)] // This is triggered in `#[derive(JsonSchema)]`

use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use secret_toolkit::utils::types::Token;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SubmitInvoice {
        purpose: String,
        amount: u128,
        admin_charge: Uint128,
        customer_charge: Uint128,
        payer: String,
        days: u64,
        recurrent_time: Option<u64>,
        token: Token,
    },
    AcceptInvoice {
        id: u64,
    },
    CancelPayment {
        id: u64,
    },
    WithdrawPayment {
        id: u64,
    },
    AdminUpdateAmin{
        newAdmin: String,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    SingleInvoice {
        id: u64,
        owner: Addr,
    },
    NumberOfInvoice {
        owner: Addr,
    },
    PaginatedInvoice {
        owner: Addr,
        page: u32,
        page_size: u32,
    },
    SingleContract {
        id: u64,
        payer: Addr,
    },
    NumberOfContract {
        payer: Addr,
    },
    PaginatedContract {
        payer: Addr,
        page: u32,
        page_size: u32,
    },
    AdmimWallet {},
}
