# Albash: privacy-preserving decentralized payroll

## Overview

- Payee provides a payment invoice to a client✅
- Payer can accept or reject an invoice from a contractor✅
- Payee can deposit on-chain assets for payment of invoices after a stipulated period✅
- Payee can cancel payment of an invoice before a critical period✅
- Payee can not cancel payment of an invoice after a critical period✅
- Payee must deposit enough assets to cover the payment of an invoice at least once✅
- Payee can pay recurrently on an invoice from an employee✅
- Payee can cancel recurrent payment✅
- Payee can accept in any token✅
- All transactions related to payment can be verified✅

## Entry Points

### Instantiate

The `instantiate` entry point is invoked upon contract deployment. It initializes the contract state by saving the admin wallet address.

### Execute

The `execute` entry point handles various execution messages related to invoice submission, acceptance, cancellation, and withdrawal.

### Query

The `query` entry point handles queries to retrieve information about invoices and contracts.

## Usage

To use the contract, follow these steps:

1. Deploy the contract to the testnet.
2. Interact with the contract using transactions to submit invoices, accept invoices, cancel payments, and withdraw payments.

## Test Contract

This repo includes test cases to ensure its functionality works as expected. These test cases cover invoice submission, invoice acceptance, and payment cancellation.

The contract includes test cases that can be run in a testing environment. These tests are designed to verify the correctness of the contract's functionality.

## Dependencies

The contract relies on the following dependencies:

- `cosmwasm_std`: Provides standard functionality for Cosmos contracts.
- `secret_toolkit`: Provides utility functions for interacting with the Secret Network.

## Functions

Instantiate
Description: Initializes the contract.

Input Parameters:

None

###

- `SubmitInvoice`
Description: Allows users to submit an invoice.

Input Parameters:

purpose: Description of the invoice.
amount: Amount to pay the invoice.
admin_charge: Payer fee for processing the invoice.
customer_charge: Payee fee for processing the invoice.
payer: Wallet address of the payer. Only a valid payer can accept an invoice.
days: Number of days before first payment.
recurrent_time: Optional. Days between recurrent payments.
token: Token used for payment.

###

`AcceptInvoice`
Description: Allows payer to accept an invoice.

Input Parameters:

id: ID of the invoice to accept.

###

`CancelPayment`
Description: Allows payer to cancel a payment.

Input Parameters:

id: ID of the payment to cancel.

###

`WithdrawPayment`
Description: Allows users to withdraw a payment.

Input Parameters:

id: ID of the payment to withdraw.

###

`AdminUpdateAmin`
Description: Allows admin to update the admin address.

Input Parameters:

newAdmin: New admin wallet address.

###

`SingleInvoice`
Description: Retrieves information about a single invoice.

Input Parameters:

id: ID of the invoice to retrieve.
owner: Wallet address of the invoice payee.

###

`NumberOfInvoice`
Description: Retrieves the number of invoices for a specific owner.

Input Parameters:

owner: Wallet address of the invoice owner.

###

`PaginatedInvoice`
Description: Retrieves a paginated list of invoices for a specific owner.

Input Parameters:

owner: Wallet address of the invoice owner.
page: Page number.
page_size: Size of each page.

###

`SingleContract`
Description: Retrieves information about a single invoice.

Input Parameters:

id: ID of the contract to retrieve.
payer: Wallet address of the invoice payer.

###

`NumberOfContract`
Description: Retrieves the number of contracts for a specific payer.

Input Parameters:

payer: Wallet address of the invoice payer.

###

`PaginatedContract`
Description: Retrieves a paginated list of contracts for a specific payer.

Input Parameters:

payer: Wallet address of the invoice payer.
page: Page number.
page_size: Size of each page.

## Contributors

- [Kombi](https://github.com/cenwadike)

- [Akin](https://github.com/come-senusi-wale)
