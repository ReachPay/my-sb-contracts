use crate::utils::AsBytes;

pub static CRYPTO_DEPOSIT_TOPIC_NAME: &'static str = "crypto-deposit";

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
    #[prost(string, tag = "6")]
    pub from_address: String,
    #[prost(string, tag = "7")]
    pub deposit_address: String,
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
