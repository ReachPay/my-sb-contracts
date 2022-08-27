use crate::utils::*;

pub static PROCESS_ORDER_COMMAND_TOPIC_NAME: &'static str = "process-order-command";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessOrderComand {
    #[prost(string, tag = "1")]
    pub order_id: String,
    #[prost(string, tag = "2")]
    pub process_id: String,
}

impl AsBytes for ProcessOrderComand {
    fn as_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result).unwrap();
        result
    }
}

impl FromBytes for ProcessOrderComand {
    fn from_bytes(src: &[u8]) -> Self {
        prost::Message::decode(src).unwrap()
    }
}
