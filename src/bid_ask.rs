use crate::utils::AsBytes;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidAskProtobufModel {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub source_date_time: i64,
    #[prost(sint64, tag = "3")]
    pub our_date_time: i64,
    #[prost(double, tag = "4")]
    pub bid: f64,
    #[prost(double, tag = "5")]
    pub ask: f64,
    #[prost(string, tag = "6")]
    pub source: ::prost::alloc::string::String,
}

impl BidAskProtobufModel {
    pub fn as_protobuf_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result)?;
        Ok(result)
    }

    pub fn from_protobuf_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        prost::Message::decode(bytes)
    }
}

impl AsBytes for BidAskProtobufModel {
    fn as_bytes(&self) -> Vec<u8> {
        self.as_protobuf_bytes().unwrap()
    }
}
