pub static PROCESS_ORDER_COMMAND_TOPIC_NAME: &'static str = "process-order-command";

#[derive(my_service_bus_macros::MySbEntityProtobufModel, Clone, PartialEq, ::prost::Message)]
pub struct ProcessOrderComand {
    #[prost(string, tag = "1")]
    pub order_id: String,
    #[prost(string, tag = "2")]
    pub process_id: String,
}
