use crate::address::{Address, AddressError};
use crate::private_key::{PrivateKey, PrivateKeyError};
use crate::public_key::PublicKey;
use crate::extended_private_key::ExtendedPrivateKeyError;

use std::fmt::Debug;

/// The interface for a generic transactions.
pub trait Transaction: Clone + Send + Sync + 'static {
    type Address: Address;
    type Format;
    type PrivateKey: PrivateKey;
    type PublicKey: PublicKey;
}

#[derive(Debug, Fail)]
pub enum TransactionError {
    #[fail(display = "{}", _0)]
    AddressError(AddressError),

    #[fail(display = "invalid binding signature for the transaction")]
    InvalidBindingSig(),

    #[fail(display = "{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display = "{}", _0)]
    ExtendedPrivateKeyError(ExtendedPrivateKeyError),

    #[fail(display = "{}", _0)]
    Message(String),

    #[fail(display = "invalid output address: {}", _0)]
    InvalidOutputAddress(String),

    #[fail(display = "insufficient information to craft transaction. missing: {}", _0)]
    InvalidInputs(String),

    #[fail(display = "invalid ouptut description for address: {}", _0)]
    InvalidOutputDescription(String),

    #[fail(display = "invalid script pub key for format: {}", _0)]
    InvalidScriptPubKey(String),

    #[fail(display = "invalid transaction id {:?}", _0)]
    InvalidTransactionId(usize),

    #[fail(display = "invalid chain id {:?}", _0)]
    InvalidChainId(u8),

    #[fail(display = "Null Error {:?}", _0)]
    NullError(()),

    #[fail(display = "{}", _0)]
    PrivateKeyError(PrivateKeyError),
}

impl From<&'static str> for TransactionError {
    fn from(msg: &'static str) -> Self {
        TransactionError::Message(msg.into())
    }
}

impl From<()> for TransactionError {
    fn from(error: ()) -> Self {
        TransactionError::NullError(error)
    }
}

impl From<AddressError> for TransactionError {
    fn from(error: AddressError) -> Self {
        TransactionError::AddressError(error)
    }
}

impl From<base58::FromBase58Error> for TransactionError {
    fn from(error: base58::FromBase58Error) -> Self {
        TransactionError::Crate("base58", format!("{:?}", error))
    }
}

impl From<bech32::Error> for TransactionError {
    fn from(error: bech32::Error) -> Self {
        TransactionError::Crate("bech32", format!("{:?}", error))
    }
}

impl From<ExtendedPrivateKeyError> for TransactionError {
    fn from(error: ExtendedPrivateKeyError) -> Self {
        TransactionError::ExtendedPrivateKeyError(error)
    }
}

impl From<hex::FromHexError> for TransactionError {
    fn from(error: hex::FromHexError) -> Self {
        TransactionError::Crate("hex", format!("{:?}", error))
    }
}

impl From<PrivateKeyError> for TransactionError {
    fn from(error: PrivateKeyError) -> Self {
        TransactionError::PrivateKeyError(error)
    }
}

impl From<secp256k1::Error> for TransactionError {
    fn from(error: secp256k1::Error) -> Self {
        TransactionError::Crate("secp256k1", format!("{:?}", error))
    }
}

impl From<std::io::Error> for TransactionError {
    fn from(error: std::io::Error) -> Self {
        TransactionError::Crate("std::io", format!("{:?}", error))
    }
}

impl From<uint::FromDecStrErr> for TransactionError {
    fn from(error: uint::FromDecStrErr) -> Self {
        TransactionError::Crate("uint", format!("{:?}", error))
    }
}