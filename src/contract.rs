#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{MINTER, TOKEN_SEQ, TOKEN_URI, OWNER};
use crate::{execute, query};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-soulbound";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    validate_token_uri(&msg.token_uri)?;
    let DepsMut {
        storage: store,
        api,
        ..
    } = deps;

    let minter: Addr = if let Some(minter) = msg.minter {
        api.addr_validate(&minter)?
    } else {
        info.sender.clone()
    };
    MINTER.save(store, &minter)?;
    set_contract_version(store, CONTRACT_NAME, CONTRACT_VERSION)?;
    TOKEN_SEQ.save(store, &Uint128::new(0))?;
    TOKEN_URI.save(store, &msg.token_uri)?;
    OWNER.save(store, &info.sender)?;

    let resp = Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("token_uri", msg.token_uri)
        .add_attribute("minter", minter.as_str());

    Ok(resp)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { to } => execute::mint(deps, info, to),
        ExecuteMsg::SetMinter { minter } => execute::mint(deps, info, minter),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTokenUri {} => to_binary(&query::get_token_uri(deps)?),
        QueryMsg::GetInfo {} => to_binary(&query::get_info(deps)?),
        QueryMsg::GetSoulBoundToken { soul } => to_binary(&query::get_soulbound(deps, soul)?),
    }
}

fn validate_token_uri(uri: &str) -> Result<(), ContractError> {
    if uri.starts_with("ipfs") && uri.ends_with(".json") {
        return Ok(());
    }
    Err(ContractError::InvalidTokenURI {})
}
