use cosmwasm_std::{DepsMut, MessageInfo, Response, Uint128};

use crate::{
    state::{MINTER, OWNER, SOULBOUND, TOKEN_SEQ},
    ContractError,
};

pub fn mint(deps: DepsMut, info: MessageInfo, to: String) -> Result<Response, ContractError> {
    let DepsMut {
        storage: store,
        api,
        ..
    } = deps;
    let to = api.addr_validate(&to)?;
    // check sender is minter
    if info.sender != MINTER.load(store)? {
        return Err(ContractError::Unauthorized {});
    }
    let mut total = TOKEN_SEQ.load(store)?;
    total += Uint128::new(1);

    TOKEN_SEQ.save(store, &total)?;
    SOULBOUND.save(store, to.clone(), &total)?;

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("to", to.as_str())
        .add_attribute("token", total.to_string()))
}

pub fn set_minter(
    deps: DepsMut,
    info: MessageInfo,
    minter: String,
) -> Result<Response, ContractError> {
    let minter = deps.api.addr_validate(&minter)?;
    // check sender is contract owner
    if info.sender != OWNER.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }
    // set minter
    MINTER.save(deps.storage, &minter)?;
    Ok(Response::new()
        .add_attribute("action", "set_minter")
        .add_attribute("minter", minter.as_str()))
}
