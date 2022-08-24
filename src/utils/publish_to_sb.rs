use std::collections::HashMap;

use my_service_bus_tcp_client::{MessageToPublish, MyServiceBusClient, PublishError};
use my_telemetry::MyTelemetryContext;

use super::{AsBytes, MY_TELEMETRY_HEADER};

pub async fn publish_to_topic<TPayload: AsBytes>(
    sb_client: &MyServiceBusClient,
    topic_id: &str,
    payload: TPayload,
    my_telemetry: MyTelemetryContext,
) -> Result<(), PublishError> {
    let content = payload.as_bytes();

    let mut headers = HashMap::new();
    headers.insert(
        MY_TELEMETRY_HEADER.to_string(),
        my_telemetry.process_id.to_string(),
    );

    sb_client
        .publish(
            topic_id,
            MessageToPublish {
                headers: Some(headers),
                content,
            },
        )
        .await
}
