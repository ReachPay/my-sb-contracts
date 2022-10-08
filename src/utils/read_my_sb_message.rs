use my_telemetry::MyTelemetryContext;
use prost::DecodeError;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::{FromBytes, MY_TELEMETRY_HEADER};

pub fn read_my_sb_message<TMessage: Send + Sync + Default + FromBytes>(
    msg: &MySbDeliveredMessage,
) -> Result<(TMessage, MyTelemetryContext), DecodeError> {
    let result: TMessage = TMessage::from_bytes(msg.content.as_slice());

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

    if let Some(process_id) = headers.get(MY_TELEMETRY_HEADER) {
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
