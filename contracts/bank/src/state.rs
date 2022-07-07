use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub currency: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalanceData {
    pub address: Addr,
    pub value: Uint128,
}

pub const STATE: Item<State> = Item::new("state");
pub const BALANCES: Map<String, BalanceData> = Map::new("balance");
pub const OWNER: Map<&Addr, Vec<String>> = Map::new("owner");
