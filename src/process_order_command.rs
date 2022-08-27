pub static PROCESS_ORDER_COMMAND_TOPIC_NAME: &'static str = "process-order-command";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessOrderComand {
    #[prost(string, tag = "1")]
    pub order_id: String,
}
