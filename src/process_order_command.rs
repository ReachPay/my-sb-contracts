use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "process-order-command")]
pub struct ProcessOrderCommand {
    #[prost(string, tag = "1")]
    pub order_id: String,
    #[prost(string, tag = "2")]
    pub process_id: String,
}
