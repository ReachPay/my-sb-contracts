use my_service_bus_tcp_client::subscribers::MySbDeliveredMessage;
use my_telemetry::MyTelemetryContext;
use prost::DecodeError;
use rust_extensions::date_time::DateTimeAsMicroseconds;

pub fn read_my_sb_message<TMessage: Send + Sync + Default + prost::Message>(
    msg: &MySbDeliveredMessage,
) -> Result<(TMessage, MyTelemetryContext), DecodeError> {
    let result: TMessage = prost::Message::decode(msg.content.as_slice())?;

    if msg.headers.is_none() {
        let result = (
            result,
            MyTelemetryContext {
                process_id: DateTimeAsMicroseconds::now().unix_microseconds,
            },
        );
        return Ok(result);
    }

    let headers = msg.headers.as_ref().unwrap();

    if let Some(process_id) = headers.get("process-id") {
        let process_id = process_id.parse::<i64>().unwrap();
        return Ok((result, MyTelemetryContext { process_id }));
    }

    let result = (
        result,
        MyTelemetryContext {
            process_id: DateTimeAsMicroseconds::now().unix_microseconds,
        },
    );

    Ok(result)
}
