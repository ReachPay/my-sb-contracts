use std::collections::HashMap;

use my_service_bus_tcp_client::{MessageToPublish, MyServiceBusClient, PublishError};
use my_telemetry::MyTelemetryContext;

use super::{AsBytes, MY_TELEMETRY_HEADER};

pub async fn publish_to_topic_with_retries<TPayload: AsBytes>(
    sb_client: &MyServiceBusClient,
    topic_id: &str,
    payload: &TPayload,
    my_telemetry: MyTelemetryContext,
) -> Result<(), PublishError> {
    loop {
        let content = payload.as_bytes();
        let mut headers = HashMap::new();
        headers.insert(
            MY_TELEMETRY_HEADER.to_string(),
            my_telemetry.process_id.to_string(),
        );

        let result = sb_client
            .publish(
                topic_id,
                MessageToPublish {
                    headers: Some(headers),
                    content,
                },
            )
            .await;

        if result.is_ok() {
            return Ok(());
        }

        match result.unwrap_err() {
            PublishError::NoConnectionToPublish => {
                wait_until_connection_is_restored(sb_client).await;
            }
            PublishError::Disconnected => {
                wait_until_connection_is_restored(sb_client).await;
            }
            PublishError::Other(other) => {
                return Err(PublishError::Other(other));
            }
        }
    }
}

async fn wait_until_connection_is_restored(sb_client: &MyServiceBusClient) {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    while !sb_client.has_connection() {
        my_logger::LOGGER.write_log(
            my_logger::LogLevel::Error,
            "Publish to MySb".to_string(),
            format!("No connection"),
            None,
        );
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
