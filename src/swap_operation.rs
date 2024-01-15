use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

use crate::BidAskProtobufModel;
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "swap-operation")]
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
