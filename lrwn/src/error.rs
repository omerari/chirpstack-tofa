use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("NetID expects exactly 3 bytes")]
    NetIdLength,

    #[error("EUI64 expects exactly 8 bytes")]
    Eui64Length,

    #[error("AES128Key expects exactly 16 bytes")]
    Aes128Length,

    #[error("DevAddr expects exactly 4 bytes")]
    DevAddrLength,

    #[error("DevAddrPrefix must be in the form 00000000/0")]
    DevAddrPrefixFormat,

    #[error("EUI64Prefix must be in the form 0000000000000000/0")]
    EUI64PrefixFormat,

    #[error(transparent)]
    FromHexError(#[from] hex::FromHexError),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
