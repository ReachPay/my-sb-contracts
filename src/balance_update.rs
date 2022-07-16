use crate::utils::AsBytes;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceUpdateMyProtobufModel {
    #[prost(string, tag = "1")]
    pub process_id: String,
    #[prost(sint64, tag = "2")]
    pub timestamp: i64,
    #[prost(string, tag = "3")]
    pub client_id: String,
    #[prost(message, repeated, tag = "4")]
    pub updates: Vec<BalanceUpdate>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceUpdate {
    #[prost(string, tag = "1")]
    pub wallet: String,
    #[prost(double, tag = "2")]
    pub balance: f64,
}

impl BalanceUpdateMyProtobufModel {
    pub fn as_protobuf_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result)?;
        Ok(result)
    }

    pub fn from_protobuf_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        prost::Message::decode(bytes)
    }
}

impl AsBytes for BalanceUpdateMyProtobufModel {
    fn as_bytes(&self) -> Vec<u8> {
        self.as_protobuf_bytes().unwrap()
    }
}
