#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapOperation {
    #[prost(sint64, tag = "1")]
    pub created: i64,
    #[prost(string, tag = "2")]
    pub process_id: String,
    #[prost(string, tag = "3")]
    pub client_id: String,
    #[prost(string, tag = "4")]
    pub from_wallet: String,
    #[prost(double, tag = "5")]
    pub from_amount: f64,
    #[prost(string, tag = "6")]
    pub to_wallet: String,
    #[prost(double, tag = "7")]
    pub to_amount: f64,
}

impl SwapOperation {
    pub fn as_protobuf_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result)?;
        Ok(result)
    }

    pub fn from_protobuf_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        prost::Message::decode(bytes)
    }
}
