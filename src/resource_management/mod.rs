use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use tokio::sync::RwLock;
use std::sync::Arc;
use dashmap::DashMap;

pub struct ResourceManager {
    memory_usage: AtomicU64,
    cpu_usage: AtomicU64,
    is_throttled: AtomicBool,
    cache: Arc<DashMap<String, CachedResource>>,
    performance_metrics: Arc<RwLock<PerformanceMetrics>>,
}

#[derive(Debug)]
struct CachedResource {
    data: Vec<u8>,
    expiry: std::time::Instant,
}

#[derive(Debug, Default)]
struct PerformanceMetrics {
    fps: f32,
    frame_time: f32,
    memory_usage: f64,
    cpu_usage: f64,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            memory_usage: AtomicU64::new(0),
            cpu_usage: AtomicU64::new(0),
            is_throttled: AtomicBool::new(false),
            cache: Arc::new(DashMap::new()),
            performance_metrics: Arc::new(RwLock::new(PerformanceMetrics::default())),
        }
    }

    pub async fn monitor_resources(&self) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        
        loop {
            interval.tick().await;
            
            // Update metrics
            let memory = self.get_memory_usage();
            let cpu = self.get_cpu_usage();
            
            // Update atomic values
            self.memory_usage.store(memory, Ordering::Relaxed);
            self.cpu_usage.store(cpu, Ordering::Relaxed);
            
            // Check if we need to throttle
            if memory > 90 || cpu > 80 {
                self.is_throttled.store(true, Ordering::Relaxed);
            } else {
                self.is_throttled.store(false, Ordering::Relaxed);
            }
            
            // Clean expired cache
            self.clean_cache();
        }
    }

    pub fn should_throttle(&self) -> bool {
        self.is_throttled.load(Ordering::Relaxed)
    }

    fn clean_cache(&self) {
        let now = std::time::Instant::now();
        self.cache.retain(|_, v| v.expiry > now);
    }
} 