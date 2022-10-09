#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "crypto-deposit-operation")]
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
    pub transaction_id: String,
    #[prost(string, tag = "8")]
    pub deposit_address: String,
}
