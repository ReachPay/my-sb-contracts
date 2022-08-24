use crate::utils::{AsBytes, FromBytes};

pub static CRYPTO_DEPOSIT_OPERATION_TOPIC_NAME: &'static str = "crypto-deposit-operation";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoDepositOperation {
    #[prost(sint64, tag = "1")]
    pub created: i64,
    #[prost(string, tag = "2")]
    pub process_id: String,
    #[prost(string, tag = "3")]
    pub client_id: String,
    #[prost(string, tag = "4")]
    pub asset_id: String,
    #[prost(double, tag = "5")]
    pub amount: f64,
    #[prost(double, tag = "6")]
    pub balance_after_operation: f64,
    #[prost(string, tag = "7")]
    pub from_address: String,
    #[prost(string, tag = "8")]
    pub deposit_address: String,
    #[prost(string, tag = "9")]
    pub transaction_id: String,
}

impl AsBytes for CryptoDepositOperation {
    fn as_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result).unwrap();
        result
    }
}

impl FromBytes for CryptoDepositOperation {
    fn from_bytes(src: &[u8]) -> Self {
        prost::Message::decode(src).unwrap()
    }
}
