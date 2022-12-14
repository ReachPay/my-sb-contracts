#[derive(Clone, Debug, ::prost::Enumeration)]
#[repr(i32)]
pub enum CryptoDepositCommandStatus {
    Initialized = 0,
    Completed = 1,
}

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "crypto-deposit-command")]
pub struct CryptoDepositCommand {
    #[prost(string, tag = "1")]
    pub source_transaction_id: String,
    #[prost(string, tag = "2")]
    pub asset_id: String,
    #[prost(double, tag = "3")]
    pub amount: f64,
    #[prost(uint64, tag = "4")]
    pub external_datetime: u64,
    #[prost(uint64, tag = "5")]
    pub internal_datetime: u64,
    #[prost(double, tag = "6")]
    pub usd_amount: f64,
    #[prost(enumeration = "CryptoDepositCommandStatus", tag = "7")]
    pub status: i32,
    #[prost(string, tag = "8")]
    pub transaction_hash: String,
    #[prost(string, tag = "9")]
    pub deposit_address: String,
    #[prost(string, tag = "10")]
    pub from_address: String,
    #[prost(string, tag = "11")]
    pub memo: Option<String>,
}
