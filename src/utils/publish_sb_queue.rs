use std::collections::VecDeque;

use my_telemetry::MyTelemetryContext;
use tokio::sync::Mutex;

pub struct PublishToSbQueue<T> {
    data: Mutex<VecDeque<(T, MyTelemetryContext)>>,
}

impl<T> PublishToSbQueue<T> {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(VecDeque::new()),
        }
    }

    pub async fn add(&self, item: T, my_telemetry_ctx: MyTelemetryContext) {
        self.data.lock().await.push_back((item, my_telemetry_ctx));
    }

    pub async fn get(&self) -> Option<(T, MyTelemetryContext)> {
        let mut write_access = self.data.lock().await;

        write_access.pop_front()
    }

    pub async fn return_back(&self, item: T, my_telemetry_ctx: MyTelemetryContext) {
        let mut write_access = self.data.lock().await;

        write_access.push_front((item, my_telemetry_ctx));
    }
}
