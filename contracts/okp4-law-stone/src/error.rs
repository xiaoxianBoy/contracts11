use cosmwasm_std::StdError;
use cw_utils::ParseReplyError;
use okp4_logic_bindings::error::TermParseError;
use okp4_wasm::error::CosmwasmUriError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    ParseReplyError(#[from] ParseReplyError),

    #[error("An unknown reply ID was received.")]
    UnknownReplyID,

    #[error("Cannot parse cosmwasm uri: {0}")]
    ParseCosmwasmUri(CosmwasmUriError),

    #[error("Cannot extract data from logic ask response: {0}")]
    LogicAskResponse(LogicAskResponseError),

    #[error("Only the contract admin can perform this operation.")]
    Unauthorized,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum LogicAskResponseError {
    #[error("Could not parse term: {0}")]
    Parse(TermParseError),

    #[error("Substitution error: {0}")]
    Substitution(String),

    #[error("Unexpected response: {0}")]
    Unexpected(String),

    #[error("Invalid parsed term format.")]
    UnexpectedTerm,
}
