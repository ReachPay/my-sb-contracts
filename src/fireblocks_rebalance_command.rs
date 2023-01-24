#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "fireblocks_rebalance_command")]
pub struct FireblocksRebalanceCommand {
    #[prost(string, tag = "1")]
    pub asset_id: String,
    #[prost(string, tag = "2")]
    pub vault_id: String,
}
