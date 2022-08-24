use crate::utils::{AsBytes, FromBytes};

pub static CRYPTO_DEPOSIT_COMMAND_TOPIC_NAME: &'static str = "crypto-deposit-command";

#[derive(Clone, Debug, ::prost::Enumeration)]
#[repr(i32)]
pub enum CryptoDepositCommandStatus {
    Initialized = 0,
    Completed = 1,
}

#[derive(PartialEq, ::prost::Message)]
pub struct CryptoDepositCommand {
    #[prost(string, tag = "1")]
    pub source_transaction_id: String,
    #[prost(string, tag = "2")]
    pub asset_id: String,
    #[prost(double, tag = "3")]
    pub amount: f64,
    #[prost(uint64, tag = "4")]
    pub external_datetime: u64,
    #[prost(uint64, tag = "5")]
    pub internal_datetime: u64,
    #[prost(double, tag = "6")]
    pub usd_amount: f64,
    #[prost(enumeration = "CryptoDepositCommandStatus", tag = "7")]
    pub status: i32,
    #[prost(string, tag = "8")]
    pub transaction_hash: String,
    #[prost(string, tag = "9")]
    pub deposit_address: String,
}

impl AsBytes for CryptoDepositCommand {
    fn as_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result).unwrap();
        result
    }
}

impl FromBytes for CryptoDepositCommand {
    fn from_bytes(src: &[u8]) -> Self {
        prost::Message::decode(src).unwrap()
    }
}
