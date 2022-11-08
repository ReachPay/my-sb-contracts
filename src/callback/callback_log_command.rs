#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "callback-log")]
pub struct BidAskProtobufModel {
    #[prost(int64, tag = "1")]
    pub execute_date: i64,
    #[prost(int32, tag = "2")]
    pub http_status_code: i32,
    #[prost(string, tag = "3")]
    pub message: String,
    #[prost(string, tag = "4")]
    pub callback_url: String,
    #[prost(message, tag = "5")]
    pub client_id: Option<String>,
    #[prost(message, tag = "6")]
    pub order_id: Option<String>,
    #[prost(string, tag = "7")]
    pub merchant_id: String,
    #[prost(string, tag = "8")]
    pub request_body: String,
    #[prost(string, tag = "9")]
    pub response_body: String,
    #[prost(string, tag = "10")]
    pub attempt: String,
}
