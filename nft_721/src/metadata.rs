use soroban_sdk::{Env, String};
use crate::token_utils::{TokenMetadata, TokenUtils};


pub fn read_name(e: &Env) -> String {
    let util = TokenUtils::new(e);
    util.get_metadata().name
}

pub fn read_symbol(e: &Env) -> String {
    let util = TokenUtils::new(e);
    util.get_metadata().symbol
}
pub fn read_base_uri(e: &Env) -> String {
    let util = TokenUtils::new(e);
    util.get_metadata().base_uri
}

pub fn write_metadata(e: &Env, metadata: TokenMetadata) {
    let util = TokenUtils::new(e);
    util.set_metadata(&metadata);
}
