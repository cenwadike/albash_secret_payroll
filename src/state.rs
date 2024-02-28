use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, DepsMut, StdResult, Storage, Uint128};
use secret_toolkit::serialization::Json;
use secret_toolkit::storage::{Item, Keymap};
use secret_toolkit::utils::types::Token;

pub const PREFIX_INVOICE: &[u8] = b"invoice";
pub const PREFIX_CONTRACT: &[u8] = b"contract";

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct Invoice {
    pub invoice_id: u64,
    pub receiver: String,
    pub purpose: String,
    pub amount: Uint128,
    pub admin_charges: Uint128,
    pub customer_charges: Uint128,
    pub payer: String,
    pub days: u64,
    pub recurrent: Option<bool>,
    pub recurrent_times: u64,
    pub remaining_time_of_payment: u64,
    pub status: String,
    pub payment_time: u64,
    pub critical_time: u64,
    pub payment_condition: String,
    pub token: Token,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct Contract {
    pub invoice_id: u64,
    pub account_balance: u128,
    pub contract_process: String,
    pub invoice: Invoice,
    pub contract_accepted: bool,
}

const INVOICE_ID: Item<u64> = Item::new(b"invoice_id");

pub fn get_next_invoice_id(storage: &mut dyn Storage) -> StdResult<u64> {
    let new_id = match INVOICE_ID.may_load(storage)? {
        Some(id) => id + 1,
        None => 1,
    };
    INVOICE_ID.save(storage, &new_id)?;

    Ok(new_id)
}

const ADMIN_WALLET_ID: &[u8] = b"user_wallet";

pub struct  AdminStore();

impl AdminStore {
    pub fn save_admin_wallet(storage: &mut dyn Storage, wallet_address: &Addr) -> StdResult<()> {
        let address_str = wallet_address.as_str();
        storage.set(ADMIN_WALLET_ID, address_str.as_bytes());
        Ok(())
    }

    pub fn get_admin_wallet( storage: &dyn Storage) -> String {
        let raw_address = storage.get(ADMIN_WALLET_ID).unwrap_or_default();
        let wallet_address = String::from_utf8(raw_address).unwrap();

        wallet_address
    }

    pub fn update_admin_wallet(storage: &mut dyn Storage, new_wallet_address: &Addr) -> StdResult<()> {
        let new_address_str = new_wallet_address.as_str();
        storage.set(ADMIN_WALLET_ID, new_address_str.as_bytes());
        Ok(())
    }
    
}

pub static INVOICE: Keymap<u64, Invoice, Json> = Keymap::new(PREFIX_INVOICE);

pub struct InvoiceStore {}

impl InvoiceStore {
    pub fn load_invoice(store: &dyn Storage, owner: &Addr, id: u64) -> Invoice {
        INVOICE
            .add_suffix(owner.as_bytes())
            .get(store, &id.clone())
            .unwrap()
    }

    pub fn save(
        store: &mut dyn Storage,
        owner: &Addr,
        id: u64,
        invoice: &Invoice,
    ) -> StdResult<()> {
        INVOICE
            .add_suffix(owner.as_bytes())
            .insert(store, &id, invoice)
    }

    pub fn paging_invoice_list(
        store: &dyn Storage,
        owner: &Addr,
        page: u32,
        page_size: u32,
    ) -> StdResult<Vec<(u64, Invoice)>> {
        INVOICE
            .add_suffix(owner.as_bytes())
            .paging(store, page, page_size)
    }

    pub fn num_invoice(store: &dyn Storage, owner: &Addr) -> u32 {
        INVOICE
            .add_suffix(owner.as_bytes())
            .get_len(store)
            .unwrap_or(0)
    }
}

pub static CONTRACT: Keymap<u64, Contract, Json> = Keymap::new(PREFIX_CONTRACT);

pub struct ContractStore {}

impl ContractStore {
    pub fn save(
        store: &mut dyn Storage,
        payer: &Addr,
        id: u64,
        contract: &Contract,
    ) -> StdResult<()> {
        CONTRACT
            .add_suffix(payer.as_bytes())
            .insert(store, &id, contract)
    }

    pub fn load_contract(store: &dyn Storage, payer: &Addr, id: u64) -> Contract {
        CONTRACT
            .add_suffix(payer.as_bytes())
            .get(store, &id.clone())
            .unwrap()
    }

    pub fn paging_contract_list(
        store: &dyn Storage,
        payer: &Addr,
        page: u32,
        page_size: u32,
    ) -> StdResult<Vec<(u64, Contract)>> {
        CONTRACT
            .add_suffix(payer.as_bytes())
            .paging(store, page, page_size)
    }

    pub fn num_contract(store: &dyn Storage, payer: &Addr) -> u32 {
        CONTRACT
            .add_suffix(payer.as_bytes())
            .get_len(store)
            .unwrap_or(0)
    }
}
