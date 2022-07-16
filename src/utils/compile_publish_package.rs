use std::collections::HashMap;

use my_service_bus_tcp_client::MessageToPublish;
use my_telemetry::MyTelemetryContext;

pub trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

pub fn compile_publish_package<TPackage: AsBytes + Send + Sync + 'static>(
    package: TPackage,
    ctx: MyTelemetryContext,
) -> MessageToPublish {
    let mut headers = HashMap::new();
    headers.insert("process-id".to_string(), ctx.process_id.to_string());

    MessageToPublish {
        headers: Some(headers),
        content: package.as_bytes(),
    }
}
