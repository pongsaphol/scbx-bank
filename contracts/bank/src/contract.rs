#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    AccountResponse, BalanceResponse, ExecuteMsg, InstantiateMsg, QueryMsg, ReceiveMsg,
};
use crate::state::{BalanceData, State, BALANCES, OWNER, STATE};
use cw20::{Cw20Contract, Cw20ExecuteMsg, Cw20ReceiveMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:bank";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
        currency: deps.api.addr_validate(msg.currency.as_str())?,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.currency.to_string()))
}

/// ExecuteMsg is the message sent to the contract to execute.
/// It is a union of all the possible messages that the contract can handle.
/// The actual message is encoded in the data field.

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateAccount { account_name } => try_create(deps, info, account_name),
        ExecuteMsg::Receive(msg) => try_receive(deps, info, msg),
        ExecuteMsg::Withdraw { account, amount } => try_withdraw(deps, info, account, amount),
        ExecuteMsg::Transfer { from, to, amount } => try_transfer(deps, info, from, to, amount),
        ExecuteMsg::ChangeCurrency { currency } => try_change_currency(deps, info, currency),
    }
}

pub fn try_create(
    deps: DepsMut,
    info: MessageInfo,
    account_name: String,
) -> Result<Response, ContractError> {
    if BALANCES
        .may_load(deps.storage, account_name.to_owned())?
        .is_some()
    {
        return Err(ContractError::InvalidRequest(
            "Account already exists".to_string(),
        ));
    }

    BALANCES.save(
        deps.storage,
        account_name.to_owned(),
        &BalanceData {
            address: info.sender.to_owned(),
            value: Uint128::zero(),
        },
    )?;

    OWNER.update(
        deps.storage,
        &info.sender,
        |state| -> Result<Vec<String>, ContractError> {
            match state {
                None => Ok(vec![account_name.clone()]),
                Some(state) => Ok([state.clone(), vec![account_name.clone()]].concat()),
            }
        },
    )?;

    Ok(Response::new()
        .add_attribute("method", "create")
        .add_attribute("owner", info.sender)
        .add_attribute("address", account_name))
}

pub fn try_receive(
    deps: DepsMut,
    info: MessageInfo,
    wrapped: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    // cw20 address authentication
    let config = STATE.load(deps.storage)?;
    if config.currency != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let msg: ReceiveMsg = from_binary(&wrapped.msg)?;
    match msg {
        ReceiveMsg::Deposit { account } => {
            receive_deposit(deps, account, wrapped.amount, info.sender)
        }
    }
}

pub fn receive_deposit(
    deps: DepsMut,
    account: String,
    amount: Uint128,
    sender: Addr,
) -> Result<Response, ContractError> {
    BALANCES.update(
        deps.storage,
        account.to_owned(),
        |balance| -> Result<BalanceData, ContractError> {
            match balance {
                None => Err(ContractError::InvalidRequest(
                    "Account does not exist".to_string(),
                )),
                Some(balance) => Ok(BalanceData {
                    address: balance.address.to_owned(),
                    value: balance.value + amount,
                }),
            }
        },
    )?;
    Ok(Response::new()
        .add_attribute("method", "deposit")
        .add_attribute("owner", sender)
        .add_attribute("address", account)
        .add_attribute("amount", amount.to_string()))
}

pub fn try_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    account: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let balance = BALANCES.may_load(deps.storage, account.to_owned())?;

    if balance.is_none() {
        return Err(ContractError::InvalidRequest(
            "Account does not exist".to_string(),
        ));
    }

    let balance = balance.unwrap();

    if balance.address != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if balance.value < amount {
        return Err(ContractError::InvalidRequest(
            "Insufficient balance".to_string(),
        ));
    }

    BALANCES.save(
        deps.storage,
        account.to_owned(),
        &BalanceData {
            address: balance.address,
            value: balance.value - amount,
        },
    )?;

    let mut res = Response::new()
        .add_attribute("method", "withdraw")
        .add_attribute("owner", info.sender.to_owned())
        .add_attribute("address", account)
        .add_attribute("amount", amount.to_string());

    let currency = STATE.load(deps.storage)?.currency;
    let cw20 = Cw20Contract(currency);
    let msg = cw20.call(Cw20ExecuteMsg::Transfer {
        recipient: info.sender.to_string(),
        amount: amount,
    })?;

    res = res.add_message(msg);

    Ok(res)
}

pub fn try_transfer(
    deps: DepsMut,
    info: MessageInfo,
    from: String,
    to: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let balance = BALANCES.may_load(deps.storage, from.to_owned())?;

    if balance.is_none() {
        return Err(ContractError::InvalidRequest(
            "Account from does not exist".to_string(),
        ));
    }

    let balance_from = balance.unwrap();

    if balance_from.address != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if balance_from.value < amount {
        return Err(ContractError::InvalidRequest(
            "Insufficient balance".to_string(),
        ));
    }

    let balance = BALANCES.may_load(deps.storage, to.to_owned())?;

    if balance.is_none() {
        return Err(ContractError::InvalidRequest(
            "Account to does not exist".to_string(),
        ));
    }

    let balance_to = balance.unwrap();

    let fee = if balance_from.address == balance_to.address {
        Uint128::zero()
    } else {
        amount
            .checked_div(Uint128::new(100))
            .unwrap_or(Uint128::zero())
    };

    BALANCES.save(
        deps.storage,
        from.to_owned(),
        &BalanceData {
            address: balance_from.address,
            value: balance_from.value - amount,
        },
    )?;

    BALANCES.save(
        deps.storage,
        to.to_owned(),
        &BalanceData {
            address: balance_to.address,
            value: balance_to.value + amount - fee,
        },
    )?;

    Ok(Response::new()
        .add_attribute("method", "transfer")
        .add_attribute("owner", info.sender)
        .add_attribute("from", from)
        .add_attribute("to", to)
        .add_attribute("amount", amount.to_string())
        .add_attribute("fee", fee.to_string()))
}

pub fn try_change_currency(
    deps: DepsMut,
    info: MessageInfo,
    currency: String,
) -> Result<Response, ContractError> {
    let config = STATE.load(deps.storage)?;
    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    STATE.save(
        deps.storage,
        &State {
            owner: config.owner,
            currency: deps.api.addr_validate(&currency)?,
        },
    )?;
    Ok(Response::new()
        .add_attribute("method", "change_currency")
        .add_attribute("owner", info.sender)
        .add_attribute("currency", currency))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAccount { address } => to_binary(&query_account(deps, address)?),
        QueryMsg::GetBalance { account } => to_binary(&query_balance(deps, account)?),
    }
}

pub fn query_account(deps: Deps, address: String) -> StdResult<AccountResponse> {
    let addr = deps.api.addr_validate(address.as_str())?;
    let owner = OWNER.may_load(deps.storage, &addr)?;
    if let Some(account) = owner {
        Ok(AccountResponse { account })
    } else {
        Err(StdError::NotFound {
            kind: "account".to_string(),
        })
    }
}

pub fn query_balance(deps: Deps, account: String) -> StdResult<BalanceResponse> {
    let balance = BALANCES.may_load(deps.storage, account.to_owned())?;
    if let Some(balance) = balance {
        Ok(BalanceResponse {
            balance: balance.value,
        })
    } else {
        Err(StdError::NotFound {
            kind: "balance".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::from_binary;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockQuerier, MOCK_CONTRACT_ADDR,
    };
    use cw20::Cw20QueryMsg;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            currency: String::from(MOCK_CONTRACT_ADDR),
        };

        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn proper_add_account_and_deposit() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            currency: String::from(MOCK_CONTRACT_ADDR),
        };

        let creator = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), creator.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // user 1 initializes the contract

        let user = mock_info("user1", &[]);

        // user 1 adds an account

        let msg = ExecuteMsg::CreateAccount {
            account_name: String::from("Account 1"),
        };
        let _res = execute(deps.as_mut(), mock_env(), user.clone(), msg);

        let msg = QueryMsg::GetAccount {
            address: user.sender.to_owned().to_string(),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let value: AccountResponse = from_binary(&res).unwrap();
        assert_eq!(value.account.len(), 1);
        assert_eq!(value.account, vec!["Account 1"]);

        // user 1 add more accounts

        let msg = ExecuteMsg::CreateAccount {
            account_name: String::from("Account 2"),
        };
        let _res = execute(deps.as_mut(), mock_env(), user.clone(), msg);

        let msg = QueryMsg::GetAccount {
            address: user.sender.to_owned().to_string(),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let value: AccountResponse = from_binary(&res).unwrap();
        assert_eq!(value.account.len(), 2);
        assert_eq!(value.account, vec!["Account 1", "Account 2"]);
    }

    #[test]
    fn check_add_unique_account_name() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            currency: String::from(MOCK_CONTRACT_ADDR),
        };

        let creator = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), creator.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // user 1 initializes the contract

        let user = mock_info("user1", &[]);

        // user 1 adds an account

        let msg = ExecuteMsg::CreateAccount {
            account_name: String::from("Account 1"),
        };
        let res = execute(deps.as_mut(), mock_env(), user.clone(), msg);

        assert_eq!(res.is_ok(), true);

        let msg = QueryMsg::GetAccount {
            address: user.sender.to_owned().to_string(),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let value: AccountResponse = from_binary(&res).unwrap();
        assert_eq!(value.account.len(), 1);
        assert_eq!(value.account, vec!["Account 1"]);

        // user 1 add same account name again
        let msg = ExecuteMsg::CreateAccount {
            account_name: String::from("Account 1"),
        };
        let res = execute(deps.as_mut(), mock_env(), user.clone(), msg);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn should_be_able_to_deposit() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            currency: String::from(MOCK_CONTRACT_ADDR),
        };

        let creator = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), creator.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // user 1 initializes the contract

        let user = mock_info("user1", &[]);

        // user 1 adds an account

        let msg = ExecuteMsg::CreateAccount {
            account_name: String::from("Account 1"),
        };
        let _res = execute(deps.as_mut(), mock_env(), user.clone(), msg);

        // user 1 deposits some money
        let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender: String::from(MOCK_CONTRACT_ADDR),
            amount: Uint128::new(55),
            msg: to_binary(&ReceiveMsg::Deposit {
                account: String::from("Account 1"),
            })
            .unwrap(),
        });
        let currency = mock_info(MOCK_CONTRACT_ADDR, &[]);
        let _res = execute(deps.as_mut(), mock_env(), currency.clone(), msg);

        let msg = QueryMsg::GetBalance {
            account: String::from("Account 1"),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let value: BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(value.balance, Uint128::new(55));
    }

    #[test]
    fn should_check_permission_before_deposit() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            currency: String::from(MOCK_CONTRACT_ADDR),
        };

        let creator = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), creator.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // user 1 initializes the contract

        let user = mock_info("user1", &[]);

        // user 1 adds an account

        let msg = ExecuteMsg::CreateAccount {
            account_name: String::from("Account 1"),
        };
        let _res = execute(deps.as_mut(), mock_env(), user.clone(), msg);

        // user 1 deposits some money
        let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender: String::from(MOCK_CONTRACT_ADDR),
            amount: Uint128::new(55),
            msg: to_binary(&ReceiveMsg::Deposit {
                account: String::from("Account 1"),
            })
            .unwrap(),
        });
        let currency = mock_info("fake_contract", &[]);
        let res = execute(deps.as_mut(), mock_env(), currency.clone(), msg);

        assert_eq!(res.is_err(), true);

        let msg = QueryMsg::GetBalance {
            account: String::from("Account 1"),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let value: BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(value.balance, Uint128::new(0));
    }

    #[test]
    fn should_be_able_to_withdraw() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            currency: String::from(MOCK_CONTRACT_ADDR),
        };

        let creator = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), creator.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // user 1 initializes the contract

        let user = mock_info("user1", &[]);

        // user 1 adds an account

        let msg = ExecuteMsg::CreateAccount {
            account_name: String::from("Account 1"),
        };
        let _res = execute(deps.as_mut(), mock_env(), user.clone(), msg);

        // user 1 deposits some money
        let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender: String::from(MOCK_CONTRACT_ADDR),
            amount: Uint128::new(55),
            msg: to_binary(&ReceiveMsg::Deposit {
                account: String::from("Account 1"),
            })
            .unwrap(),
        });
        let currency = mock_info(MOCK_CONTRACT_ADDR, &[]);
        let _res = execute(deps.as_mut(), mock_env(), currency.clone(), msg);

        let msg = QueryMsg::GetBalance {
            account: String::from("Account 1"),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let value: BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(value.balance, Uint128::new(55));

        // user 1 withdraws some money
        let msg = ExecuteMsg::Withdraw {
            account: String::from("Account 1"),
            amount: Uint128::new(16),
        };
        let _res = execute(deps.as_mut(), mock_env(), user.clone(), msg);
        println!("{:?}", _res);
        let msg = QueryMsg::GetBalance {
            account: String::from("Account 1"),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let value: BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(value.balance, Uint128::new(39));
    }

    // #[test]
    // fn increment() {
    //     let mut deps = mock_dependencies();

    //     let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Increment {};
    //     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // should increase counter by 1
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(18, value.count);
    // }

    // #[test]
    // fn reset() {
    //     let mut deps = mock_dependencies();

    //     let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let unauth_info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
    //     match res {
    //         Err(ContractError::Unauthorized {}) => {}
    //         _ => panic!("Must return unauthorized error"),
    //     }

    //     // only the original creator can reset the counter
    //     let auth_info = mock_info("creator", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

    //     // should now be 5
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(5, value.count);
    // }
}
