use cosmwasm_std::Uint128;
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub currency: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Create a new account with the zero balance
    CreateAccount {
        /// The account name of the account to create
        account_name: String,
    },
    /// Receive forwards received cw20 tokens to an execution logic (in case of Deposit)
    Receive(Cw20ReceiveMsg),
    /// Withdraw funds from an account
    Withdraw {
        /// The account to withdraw from
        account: String,
        /// The amount to withdraw
        amount: Uint128,
    },
    /// Transfer funds from one account to another
    Transfer {
        /// The account to transfer from
        from: String,
        /// The account to transfer to
        to: String,
        /// The amount to transfer
        amount: Uint128,
    },
    /// Change the currency of an account
    ChangeCurrency {
        /// The new currency
        currency: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetAccount {
        /// address of the account to get
        address: String,
    },
    GetBalance {
        /// The account to get the balance of
        account: String,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AccountResponse {
    pub account: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalanceResponse {
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveMsg {
    /// Receive forwards received cw20 tokens to an execution logic (in case of Deposit)
    Deposit { account: String },
}
