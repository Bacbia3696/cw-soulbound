use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub minter: Option<String>,
    pub token_uri: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint { to: String },
    SetMinter { minter: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetTokenURIResponse)]
    GetTokenURI {},
    #[returns(GetInfoResponse)]
    GetInfo {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}

#[cw_serde]
pub struct GetTokenURIResponse {
    pub token_uri: String,
}

#[cw_serde]
pub struct GetInfoResponse {
    pub minter: Addr,
    pub token_uri: String,
    pub owner: Addr,
    pub token_total: Uint128,
}
