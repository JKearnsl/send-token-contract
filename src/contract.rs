use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::entry_point;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SendTokens { to} => execute::send_tokens(info, to),
    }
}

pub mod execute {
    use cosmwasm_std::{Addr, BankMsg};

    use super::*;

    pub fn send_tokens(info: MessageInfo, to: Addr) -> Result<Response, ContractError> {
        if info.funds.is_empty() {
            return Err(ContractError::NoFundsSent {});
        }

        Ok(Response::new()
            .add_attribute("action", "send_tokens")
            .add_attribute("sender", info.sender)
            .add_attribute("recipient", to.clone())
            .add_message(BankMsg::Send {
                to_address: to.into_string(),
                amount: info.funds.clone()
            })
        )
    }
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
}

pub mod query {}

#[cfg(test)]
mod tests {}
