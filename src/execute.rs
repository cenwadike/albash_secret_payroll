use cosmwasm_std::{
    coins, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};

use secret_toolkit::utils::types::Token;

use crate::state::{get_next_invoice_id, AdminStore, Contract, ContractStore, Invoice, InvoiceStore};

pub struct Empty {}

#[allow(clippy::too_many_arguments)]
pub fn new_invoice(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    purpose: String,
    amount: u128,
    admin_charge: Uint128,
    customer_charge: Uint128,
    payer: String,
    days: u64,
    recurrent_time: Option<u64>,
    token: Token,
) -> StdResult<Response> {
    // get the signer
    let receiver = info.sender;

    // validate payer address
    let payer_address = deps.api.addr_validate(payer.as_str())?;

    // get next invoice id
    let next_invoice_id = get_next_invoice_id(deps.storage)?;

    // check if recurrent time is specify
    let times_of_recurrent = match recurrent_time {
        Some(time) => time,
        None => 0,
    };

    if admin_charge < Uint128::new(1) {
        let error_message = format!(
            "charge on payer must be greater than 0",
        );
    
        return Err(StdError::generic_err(
            error_message
        ));
    }

    if customer_charge < Uint128::new(1) {
        let error_message = format!(
            "charge on payee must be greater than 0",
        );
    
        return Err(StdError::generic_err(
            error_message
        ));
    }

    let recurrent_status = match recurrent_time {
        Some(_time) => true,
        None => false,
    };

    let status = "not started".to_string();

    let invoice = Invoice {
        invoice_id: next_invoice_id,
        receiver: receiver.to_string(),
        purpose: purpose,
        amount: amount.into(),
        admin_charges: admin_charge,
        customer_charges: customer_charge,
        payer: payer_address.to_string(),
        days: days,
        recurrent: Some(recurrent_status),
        recurrent_times: times_of_recurrent,
        remaining_time_of_payment: 0,
        status: status,
        payment_time: 0,
        critical_time: 0,
        payment_condition: "no".to_string(),
        token: token,
    };

    InvoiceStore::save(deps.storage, &receiver, next_invoice_id, &invoice)?;

    let contract = Contract {
        invoice_id: next_invoice_id,
        account_balance: 0,
        contract_process: "not started".to_string(),
        invoice: invoice,
        contract_accepted: false,
    };

    ContractStore::save(deps.storage, &payer_address, next_invoice_id, &contract)?;

    deps.api.debug("invoice created successfully");
    Ok(Response::default())
}

pub fn accept_invoice(deps: DepsMut, env: Env, info: MessageInfo, id: u64) -> StdResult<Response> {
    // get payer address from function param
    let payer = info.sender;

    // get the contract of specific id related to invoice
    let mut contract = ContractStore::load_contract(deps.storage, &payer, id);

    // get invoice of specific id in related to contract
    let mut invoice = &mut contract.invoice;

    // get the receiver address
    let receiver = deps.api.addr_validate(invoice.receiver.as_str())?;

    let payer_address = deps.api.addr_validate(invoice.payer.as_str())?;

    // verify that the payer in invoice
    if payer_address != payer {
        return Err(StdError::generic_err(
            "You are not the payer of this Invoice",
        ));
    }

    if contract.contract_accepted == true {
        return Err(StdError::generic_err("Invoice have already been accepted"));
    }

    let mut amount = Uint128::zero();

    let mut admin_withraw_amount = Uint128::zero();

    // get admin wallet address
    let admin_wallet = AdminStore::get_admin_wallet( deps.storage);

    // validate admin wallet addres
    let admin_wallet_validate = deps.api.addr_validate(admin_wallet.as_str())?;

    let denom = "uscrt".to_string();

    if invoice.recurrent == Some(true) {
        let expected_amount = invoice.amount * Uint128::new(invoice.recurrent_times.into());

        let total_admin_charges = invoice.admin_charges * Uint128::new(invoice.recurrent_times.into());

        let total_expected_amount = expected_amount + total_admin_charges;

        admin_withraw_amount = total_admin_charges;

        for coin in &info.funds {
            amount += coin.amount
        }

        if amount < total_expected_amount {
            let error_message = format!(
                "Amount {} is insufficient for recurrent payment in Invoice. Expected amount: {}",
                amount, total_expected_amount
            );
        
            return Err(StdError::generic_err(
                error_message
            ));
        }

    } else {
        let total_admin_charges_single = invoice.admin_charges;

        let total_expected_amount_single = invoice.amount + total_admin_charges_single;

        admin_withraw_amount = total_admin_charges_single;

        for coin in &info.funds {
            amount += coin.amount
        }

        if amount < total_expected_amount_single {
            let error_message = format!(
                "Amount {} is insufficient for recurrent payment in Invoice. Expected amount: {}",
                amount, total_expected_amount_single
            );
          
            return Err(StdError::generic_err(
                error_message
            ));
        }
    }

    if amount.is_zero() {
        return Err(StdError::generic_err("Insufficient token attach"));
    }

    let remaining_time_of_payment = match invoice.recurrent {
        Some(_remaining_time) => invoice.recurrent_times,
        None => 1,
    };

    // transfer admin money to his wallet
     CosmosMsg::<Empty>::Bank(BankMsg::Send {
        to_address: admin_wallet_validate.to_string(),
        amount: coins(admin_withraw_amount.into(), denom),
    });

    let account_balance = amount - admin_withraw_amount;

    let current_block_time = env.block.time.seconds();
    let day_in_timestamp = invoice.days * 86400;
    let paid_time = current_block_time + day_in_timestamp;
    let critical_time = paid_time / 2;

    // updating invoice field
    invoice.payment_time = paid_time;
    invoice.critical_time = critical_time;
    invoice.payment_condition = "pay full".to_string();
    invoice.status = "accepted".to_string();
    invoice.remaining_time_of_payment = remaining_time_of_payment;

    contract.account_balance = account_balance.into();
    contract.contract_accepted = true;
    contract.contract_process = "started".to_string();

    // save the update
    InvoiceStore::save(deps.storage, &receiver, id, &invoice)?;
    ContractStore::save(deps.storage, &payer, id, &contract)?;

    deps.api.debug("invoice accepted successfully");
    Ok(Response::default())
}

pub fn stop_contract(deps: DepsMut, env: Env, info: MessageInfo, id: u64) -> StdResult<Response> {
    // get the signer which is the payer
    let payer = info.sender;

    // get the contract of specific id related to invoice
    let mut contract = ContractStore::load_contract(deps.storage, &payer, id);

    // get invoice of specific id in related to contract
    let mut invoice = &mut contract.invoice;

    // get the receiver address
    let receiver = deps.api.addr_validate(invoice.receiver.as_str())?;

    let payer_address = deps.api.addr_validate(invoice.payer.as_str())?;

    // verify the signer in invoice
    if payer_address != payer {
        return Err(StdError::generic_err(
            "You are not the payer for this invoice",
        ));
    }

    // check if the payer has accepted the contract
    if contract.contract_accepted != true {
        return Err(StdError::generic_err("You have not accepted this invoice"));
    }

    // check if the contract has been carry out
    if contract.contract_process == "done".to_string() {
        return Err(StdError::generic_err(
            "The purpose of the invoice have been marked as DONE",
        ));
    }

    // check if th contract has been stop already
    if contract.contract_process == "stop".to_string() {
        return Err(StdError::generic_err("Invoice have already been canceled"));
    }

    let current_block_time = env.block.time.seconds();

    let denom = "uscrt".to_string();

    if invoice.critical_time > current_block_time {
        // set the amount to half of current payment
        let amount_to_pay = invoice.amount / Uint128::new(2);

        // get the remaining balance
        let remaining_balance: u128 =
            contract.account_balance - <Uint128 as Into<u128>>::into(amount_to_pay);

        // payer should receive their remaining balance
        CosmosMsg::<Empty>::Bank(BankMsg::Send {
            to_address: payer.to_string(),
            amount: coins(remaining_balance, denom),
        });

        invoice.payment_condition = "half".to_string();
        invoice.amount = amount_to_pay;
        invoice.status = "stop".to_string();
        invoice.remaining_time_of_payment = 1;

        contract.contract_process = "stop".to_string();
        contract.account_balance = amount_to_pay.into();

        // save the update
        InvoiceStore::save(deps.storage, &receiver, id, &invoice)?;
        ContractStore::save(deps.storage, &payer, id, &contract)?;
    } else {
        // payer should receive all pending their money back
        CosmosMsg::<Empty>::Bank(BankMsg::Send {
            to_address: payer.to_string(),
            amount: coins(contract.account_balance, denom),
        });

        invoice.payment_condition = "no".to_string();
        invoice.amount = Uint128::new(0);
        invoice.status = "stop".to_string();
        invoice.remaining_time_of_payment = 0;

        contract.contract_process = "stop".to_string();
        contract.account_balance = 0;

        // save the update
        InvoiceStore::save(deps.storage, &receiver, id, &invoice)?;
        ContractStore::save(deps.storage, &payer, id, &contract)?;
    }

    deps.api.debug("invoice canceled successfully");
    Ok(Response::default())
}

pub fn withdraw_payment(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: u64,
) -> StdResult<Response> {
    // get the signer which is the receiver of payment
    let receiver = info.sender;

    // get invoice of specific id in related to contract
    let mut invoice = InvoiceStore::load_invoice(deps.storage, &receiver, id);

    // receiver address in the invoice
    let receiver_address = deps.api.addr_validate(invoice.receiver.as_str())?;

    // verify payer address in the invoice
    let payer = deps.api.addr_validate(invoice.payer.as_str())?;

    // check that the signer is one that submitted the invoice
    if receiver_address != receiver {
        return Err(StdError::generic_err(
            "You are not the payee of this invoice",
        ));
    }

    // get the contract of specific id related to invoice
    let contract = ContractStore::load_contract(deps.storage, &payer, id);

    // check if the contract has been accepted
    if contract.contract_accepted != true {
        return Err(StdError::generic_err("Invoice have not been accepted"));
    }

    let current_block_time = env.block.time.seconds();

    if current_block_time < invoice.payment_time {
        return Err(StdError::generic_err(
            "Payment period have not been reached for this",
        ));
    }

    if invoice.remaining_time_of_payment == 0 {
        return Err(StdError::generic_err(
            "All payment have been made for this invoice",
        ));
    }

    if invoice.payment_condition == "no".to_string() {
        return Err(StdError::generic_err("Invoice have been canceled"));
    }

    // check if the payer has money in is account
    if contract.account_balance < 1 {
        return Err(StdError::generic_err(
            "No payment reserved for this Invoice",
        ));
    }

    let denom = "uscrt".to_string();

    // get admin wallet address
    let admin_wallet = AdminStore::get_admin_wallet( deps.storage);

    // validate admin wallet addres
    let admin_wallet_validate = deps.api.addr_validate(admin_wallet.as_str())?;

    if invoice.payment_condition == "half".to_string() {
        invoice.status = "done".to_string();
        invoice.remaining_time_of_payment = 0;

        let changes = invoice.customer_charges;

        let payee_payment = invoice.amount - changes;

        // save invoice changes
        InvoiceStore::save(deps.storage, &receiver, id, &invoice)?;

        let contract_store = Contract {
            invoice_id: id,
            account_balance: 0,
            contract_process: contract.contract_process,
            invoice,
            contract_accepted: contract.contract_accepted,
        };

        // employee receive their payment
        CosmosMsg::<Empty>::Bank(BankMsg::Send {
            to_address: receiver.to_string(),
            amount: coins(payee_payment.into(), denom.clone()),
        });

        // admin receive his changes
        CosmosMsg::<Empty>::Bank(BankMsg::Send {
            to_address: admin_wallet_validate.to_string(),
            amount: coins(changes.into(), denom.clone()),
        });

        // save contract changes
        ContractStore::save(deps.storage, &payer, id, &contract_store)?;
    } else {
        //get the remaining time
        let remaining_time_of_payment = match invoice.recurrent {
            Some(_remaining_time) => invoice.remaining_time_of_payment - 1,
            None => 0,
        };

        let account_balance =
            contract.account_balance - <Uint128 as Into<u128>>::into(invoice.amount);

        invoice.remaining_time_of_payment = remaining_time_of_payment;

        let changes = invoice.customer_charges;

        let payee_payment = invoice.amount - changes;

        // save invoice changes
        InvoiceStore::save(deps.storage, &receiver, id, &invoice)?;

        let contract_store = Contract {
            invoice_id: id,
            account_balance: account_balance,
            contract_process: contract.contract_process,
            invoice: invoice,
            contract_accepted: contract.contract_accepted,
        };

        // employee receive their payment
        CosmosMsg::<Empty>::Bank(BankMsg::Send {
            to_address: receiver.to_string(),
            amount: coins(payee_payment.into(), denom.clone()),
        });

        // admin receive his changes
        CosmosMsg::<Empty>::Bank(BankMsg::Send {
            to_address: admin_wallet_validate.to_string(),
            amount: coins(changes.into(), denom.clone()),
        });

        // save contract changes
        ContractStore::save(deps.storage, &payer, id, &contract_store)?;
    }

    deps.api.debug("invoice accepted successfully");
    Ok(Response::default())
}

pub fn admin_change_admin(deps: DepsMut, env: Env, info: MessageInfo, admin: String) -> StdResult<Response> {
    // get the signer which is the payer
    let sender = info.sender;

    let admin_wallet = AdminStore::get_admin_wallet( deps.storage);

    let admin_wallet_validate = deps.api.addr_validate(admin_wallet.as_str())?;

    // check the signer ias admin
    if sender != admin_wallet_validate {
        return Err(StdError::generic_err(
            "Admin role only",
        ));
    }

    // validate new admin adress address
    let new_admin_address = deps.api.addr_validate(admin.as_str())?;

    //save new admin address
    AdminStore::update_admin_wallet(deps.storage, &new_admin_address)?;

    deps.api.debug("new admin save successfully");
    Ok(Response::default())
}

