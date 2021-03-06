use crate::{utils::AsBytes, BidAskProtobufModel};

pub static SWAP_TOPIC_NAME: &'static str = "swap-operation";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapOperation {
    #[prost(sint64, tag = "1")]
    pub created: i64,
    #[prost(string, tag = "2")]
    pub process_id: String,
    #[prost(string, tag = "3")]
    pub client_id: String,
    #[prost(string, tag = "4")]
    pub sell_asset: String,
    #[prost(double, tag = "5")]
    pub sell_amount: f64,
    #[prost(double, tag = "6")]
    pub sell_asset_balance: f64,
    #[prost(string, tag = "7")]
    pub buy_asset: String,
    #[prost(double, tag = "8")]
    pub buy_amount: f64,
    #[prost(double, tag = "9")]
    pub buy_asset_balance: f64,
    #[prost(message, repeated, tag = "10")]
    pub bid_asks: Vec<BidAskProtobufModel>,
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

impl AsBytes for SwapOperation {
    fn as_bytes(&self) -> Vec<u8> {
        self.as_protobuf_bytes().unwrap()
    }
}
