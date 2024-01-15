use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "withdrawals-status-update")]
pub struct WithdrawalStatusUpdate {
    #[prost(sint64, tag = "1")]
    pub happened: i64,
    #[prost(string, tag = "2")]
    pub id: String,
    #[prost(string, tag = "3")]
    pub client_id: String,
    #[prost(double, tag = "4")]
    pub amount: f64,
    #[prost(string, tag = "5")]
    pub currency: String,
    #[prost(string, tag = "6")]
    pub status: String,
}
