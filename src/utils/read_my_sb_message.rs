use my_service_bus_tcp_client::subscribers::MySbDeliveredMessage;
use my_telemetry::MyTelemetryContext;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::AsBytes;

pub fn read_my_sb_message<TMessage: AsBytes>(
    msg: MySbDeliveredMessage,
) -> (TMessage, MyTelemetryContext) {
    let result = TMessage::from_bytes(msg.content.as_slice());

    if msg.headers.is_none() {
        return (
            result,
            MyTelemetryContext {
                process_id: DateTimeAsMicroseconds::now().unix_microseconds,
            },
        );
    }

    let headers = msg.headers.as_ref().unwrap();

    if let Some(process_id) = headers.get("process-id") {
        let process_id = process_id.parse::<i64>().unwrap();
        return (result, MyTelemetryContext { process_id });
    }

    (
        result,
        MyTelemetryContext {
            process_id: DateTimeAsMicroseconds::now().unix_microseconds,
        },
    )
}
