use crate::utils::AsBytes;

pub static CRYPTO_DEPOSIT_OPERATION_TOPIC_NAME: &'static str = "crypto-deposit-operation";

#[derive(Clone, Debug, ::prost::Enumeration)]
#[repr(i32)]
pub enum CryptoDepositOperationStatus {
    Initialized = 0,
    Completed = 1,
}

#[derive(PartialEq, ::prost::Message)]
pub struct CryptoDepositOperation {
    #[prost(string, tag = "1")]
    pub source_transaction_id: String,
    #[prost(string, tag = "2")]
    pub wallet_id: String,
    #[prost(double, tag = "3")]
    pub amount: f64,
    #[prost(uint64, tag = "4")]
    pub external_datetime: u64,
    #[prost(uint64, tag = "5")]
    pub internal_datetime: u64,
    #[prost(double, tag = "6")]
    pub usd_amount: f64,
    #[prost(enumeration = "CryptoDepositOperationStatus", tag = "7")]
    pub status: i32,
    #[prost(string, tag = "8")]
    pub transaction_hash: String,
}

impl CryptoDepositOperation {
    pub fn as_protobuf_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result)?;
        Ok(result)
    }

    pub fn from_protobuf_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        prost::Message::decode(bytes)
    }
}

impl AsBytes for CryptoDepositOperation {
    fn as_bytes(&self) -> Vec<u8> {
        self.as_protobuf_bytes().unwrap()
    }
}
