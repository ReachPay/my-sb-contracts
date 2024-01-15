use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "bid-ask")]
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
