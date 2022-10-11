#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoDepositCommission {
    #[prost(double, tag = "1")]
    pub commission: f64,
    #[prost(string, tag = "2")]
    pub commission_client_id: String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "crypto-deposit-operation")]
pub struct CryptoDepositOperation {
    #[prost(sint64, tag = "1")]
    pub created: i64,
    #[prost(string, tag = "2")]
    pub process_id: String,
    #[prost(string, tag = "3")]
    pub client_id: String,
    #[prost(string, optional, tag = "4")]
    pub merchant_id: Option<String>,
    #[prost(string, tag = "5")]
    pub asset_id: String,
    #[prost(double, tag = "6")]
    pub amount: f64,
    #[prost(double, tag = "7")]
    pub balance_after_operation: f64,
    #[prost(string, tag = "8")]
    pub transaction_id: String,
    #[prost(string, tag = "9")]
    pub deposit_address: String,
    #[prost(message, tag = "10")]
    pub commission: Option<CryptoDepositCommission>,
}
