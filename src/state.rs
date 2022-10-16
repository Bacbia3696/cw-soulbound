use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

/// contract owner, who can set minter and migrate contract
pub const OWNER: Item<Addr> = Item::new("owner");
/// address can mint new soulboundtoken
pub const MINTER: Item<Addr> = Item::new("minter");
// total number of token
pub const TOKEN_SEQ: Item<Uint128> = Item::new("token_seq");
/// Store url that describe soulboundtoken
/// should exp ipfs://bafybeigtyxrp4ujy4u2juthjfc26blvu7lnxjkwcac4stahn3ixayhxwe4/9036.json
pub const TOKEN_URI: Item<String> = Item::new("meta");

pub const SOULBOUND: Map<Addr, Uint128> = Map::new("soulbound");
