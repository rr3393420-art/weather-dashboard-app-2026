use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, Duration};
use crate::weather::Weather;

#[derive(Clone)]
pub struct CacheEntry {
    data: Weather,
    timestamp: SystemTime,
}

pub struct Cache {
    data: Arc<Mutex<HashMap<String, CacheEntry>>>,
    ttl: Duration,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            data: Arc::new(Mutex::new(HashMap::new())),
            ttl: Duration::from_secs(600), // 10 minutes TTL
        }
    }

    pub fn get(&self, key: &str) -> Option<Weather> {
        let cache = self.data.lock().unwrap();
        
        if let Some(entry) = cache.get(key) {
            if let Ok(elapsed) = entry.timestamp.elapsed() {
                if elapsed < self.ttl {
                    return Some(entry.data.clone());
                }
            }
        }
        
        None
    }

    pub fn set(&self, key: String, value: Weather) {
        let mut cache = self.data.lock().unwrap();
        cache.insert(
            key,
            CacheEntry {
                data: value,
                timestamp: SystemTime::now(),
            },
        );
    }

    pub fn clear(&self) {
        let mut cache = self.data.lock().unwrap();
        cache.clear();
    }
}
