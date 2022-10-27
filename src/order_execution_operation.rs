#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderExecutionStep {
    #[prost(string, tag = "1")]
    pub from_client_id: String,
    #[prost(string, tag = "2")]
    pub to_client_id: String,
    #[prost(double, tag = "3")]
    pub delta: f64,
    #[prost(double, tag = "4")]
    pub balance_after_operation: f64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderExecutionCommissionGrpcModel {
    #[prost(double, tag = "1")]
    pub commission: f64,
    #[prost(bool, tag = "2")]
    pub is_commission_on_top: bool,
    #[prost(string, tag = "3")]
    pub comission_client_id: String,
    #[prost(double, tag = "4")]
    pub balance_after_commission_deposit: f64,
}

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
    pub currency: String,
    #[prost(message, repeated, tag = "5")]
    pub steps: Vec<OrderExecutionStep>,
    #[prost(message, tag = "6")]
    pub commission: Option<OrderExecutionCommissionGrpcModel>,
}
