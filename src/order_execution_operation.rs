#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "order-execution-operation")]
pub struct OrderExecutionOperation {
    #[prost(sint64, tag = "1")]
    pub created: i64,
    #[prost(string, tag = "2")]
    pub process_id: String,
    #[prost(string, tag = "3")]
    pub order_id: String,
    #[prost(string, tag = "4")]
    pub from_client_id: String,
    #[prost(string, tag = "5")]
    pub to_client_id: String,
    #[prost(string, tag = "6")]
    pub currency: String,
    #[prost(double, tag = "7")]
    pub amount: f64,
    #[prost(double, tag = "8")]
    pub from_balance_after_operation: f64,
    #[prost(double, tag = "9")]
    pub to_balance_after_operation: f64,
    #[prost(double, tag = "10")]
    pub commission: f64,
}
