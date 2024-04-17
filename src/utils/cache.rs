use std::collections::HashMap;
use std::future::Future;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Sub;
use std::sync::Mutex;

use chrono::{DateTime, TimeDelta, Utc};
use dioxus::prelude::Readable;
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;

const LONGEST_REMAIN_TIME: i64 = 1800;
// longest cache time unit is second
lazy_static! {
    static ref CACHE: Mutex<Cache> ={
       let cache = Cache::init();
        Mutex::new(cache)
    };
}
#[derive(Clone)]
pub struct Cache{
    last_update_time: i64,
    cache: HashMap<u128,Vec<u8>>,
}

impl Cache {
    pub fn get_cache(&mut self, uuid: u128) -> Option<Vec<u8>> {
        let now = Utc::now();
        if now
            .sub(TimeDelta::seconds(LONGEST_REMAIN_TIME))
            .ge(&DateTime::from_timestamp_millis(self.last_update_time).unwrap())
        {
            return None;
        }
        return self.cache.get(&uuid).cloned();
    }

    pub fn set_cache(&mut self, uuid: u128, hashCode: Vec<u8>) {
        let now = Utc::now().timestamp_millis();
        self.cache.insert(uuid, hashCode);
        self.last_update_time = now;
    }

    pub fn init() -> Self {
        return  Cache {
            last_update_time: Utc::now().timestamp_millis(),
            cache: HashMap::new(),
        };
    }
    pub async fn fetch_or_cache<T: DeserializeOwned, Fut>(
        url: &str,
        fn_after_fetch: impl Fn() -> Fut,
    ) -> serde_json::Result<T>
    where
        Fut: Future<Output = Vec<u8>>,
    {
        // first try to get from cache
        let mut hasher = DefaultHasher::new();
        url.hash(&mut hasher);
        let uuid = hasher.finish() as u128;
        let result = CACHE.lock().unwrap().get_cache(uuid);
        let response = match result {
            None => {
                // if no cache, use fn to fetch then cache it
                let response = fn_after_fetch().await;
                CACHE.lock().unwrap().set_cache(uuid, response.clone());
                response
            }
            Some(response) => response,
        };
        serde_json::from_slice::<T>(&*response)
    }
}
