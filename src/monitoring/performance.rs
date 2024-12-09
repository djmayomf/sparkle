use metrics::{Counter, Gauge, Histogram, Key, KeyName, Unit};
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::time::{interval, Duration};

pub struct PerformanceMonitor {
    frame_time: Histogram,
    memory_usage: Gauge,
    cpu_usage: Gauge,
    error_count: Counter,
    request_latency: Histogram,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            frame_time: Histogram::new(Key::from("frame_time"), Unit::Milliseconds),
            memory_usage: Gauge::new(Key::from("memory_usage"), Unit::Bytes),
            cpu_usage: Gauge::new(Key::from("cpu_usage"), Unit::Percent),
            error_count: Counter::new(Key::from("errors")),
            request_latency: Histogram::new(Key::from("request_latency"), Unit::Milliseconds),
        }
    }

    pub async fn start_monitoring(&self) {
        let mut interval = interval(Duration::from_secs(1));
        
        loop {
            interval.tick().await;
            self.collect_metrics().await;
        }
    }

    async fn collect_metrics(&self) {
        // Update metrics
        self.memory_usage.set(self.get_memory_usage() as f64);
        self.cpu_usage.set(self.get_cpu_usage() as f64);
        
        // Record frame time
        if let Some(frame_time) = self.measure_frame_time().await {
            self.frame_time.record(frame_time);
        }
    }
} 