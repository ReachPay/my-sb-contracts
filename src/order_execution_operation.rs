use crate::utils::{AsBytes, FromBytes};

pub static ORDER_EXECUTION_OPERATION_TOPIC_NAME: &'static str = "order-execution-operation";
#[derive(Clone, PartialEq, ::prost::Message)]
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
}

impl AsBytes for OrderExecutionOperation {
    fn as_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result).unwrap();
        result
    }
}

impl FromBytes for OrderExecutionOperation {
    fn from_bytes(src: &[u8]) -> Self {
        prost::Message::decode(src).unwrap()
    }
}
