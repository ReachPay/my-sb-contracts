pub static BALANCE_UPDATE_TOPIC_NAME: &'static str = "balance-update";

#[derive(my_service_bus_macros::MySbEntityProtobufModel, Clone, PartialEq, ::prost::Message)]
pub struct BalanceUpdateProtobufModel {
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
