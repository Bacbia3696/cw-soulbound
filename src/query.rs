use cosmwasm_std::{Deps, StdResult};

use crate::{
    msg::{GetInfoResponse, GetTokenURIResponse},
    state::{MINTER, OWNER, TOKEN_SEQ, TOKEN_URI},
};

pub fn get_token_uri(deps: Deps) -> StdResult<GetTokenURIResponse> {
    let token_uri = TOKEN_URI.load(deps.storage)?;
    Ok(GetTokenURIResponse { token_uri })
}

pub fn get_info(deps: Deps) -> StdResult<GetInfoResponse> {
    let token_uri = TOKEN_URI.load(deps.storage)?;
    let minter = MINTER.load(deps.storage)?;
    let owner = OWNER.load(deps.storage)?;
    let token_total = TOKEN_SEQ.load(deps.storage)?;
    Ok(GetInfoResponse {
        minter,
        owner,
        token_uri,
        token_total,
    })
}
