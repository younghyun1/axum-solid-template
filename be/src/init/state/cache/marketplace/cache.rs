use std::{path::PathBuf, time::Duration};

use chrono::{DateTime, Utc};
use moka::future::Cache;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::{fs, task};
use tracing::info;

use crate::init::{
    server_config::marketplace_config::MarketplaceConfig,
    state::cache::marketplace::error::{
        MarketplacePublicCacheError, io_error, serialization_error,
    },
};

const DISK_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone)]
pub struct MarketplacePublicCache {
    entries: Cache<String, String>,
    cache_dir: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
struct DiskCacheEntry {
    schema_version: u32,
    key: String,
    value: String,
    stored_at: DateTime<Utc>,
}

impl MarketplacePublicCache {
    pub async fn load(config: &MarketplaceConfig) -> Result<Self, MarketplacePublicCacheError> {
        let cache_dir = PathBuf::from(&config.cache_dir);
        match fs::create_dir_all(&cache_dir).await {
            Ok(()) => {}
            Err(error) => return Err(io_error(error)),
        }

        let entries = Cache::builder()
            .max_capacity(config.cache_capacity)
            .time_to_live(Duration::from_secs(config.cache_ttl_seconds))
            .build();
        let cache = Self { entries, cache_dir };
        let loaded_entries = match cache.hydrate_from_disk().await {
            Ok(count) => count,
            Err(error) => return Err(error),
        };

        info!(
            loaded_entries,
            cache_dir = %config.cache_dir,
            capacity = config.cache_capacity,
            ttl_seconds = config.cache_ttl_seconds,
            "Loaded marketplace public cache"
        );

        Ok(cache)
    }

    pub async fn get_json(&self, key: &str) -> Option<String> {
        match self.entries.get(key).await {
            Some(value) => Some(value),
            None => self.read_disk_entry(key).await,
        }
    }

    pub async fn put_json(
        &self,
        key: String,
        value: String,
    ) -> Result<(), MarketplacePublicCacheError> {
        self.entries.insert(key.clone(), value.clone()).await;
        self.write_disk_entry(key, value).await
    }

    pub async fn clear(&self) -> Result<(), MarketplacePublicCacheError> {
        self.entries.invalidate_all();
        self.entries.run_pending_tasks().await;
        let cache_dir = self.cache_dir.clone();
        let result = task::spawn_blocking(move || remove_cache_files(cache_dir)).await;
        match result {
            Ok(remove_result) => remove_result,
            Err(error) => Err(MarketplacePublicCacheError::TaskJoin {
                error: error.to_string(),
            }),
        }
    }

    async fn hydrate_from_disk(&self) -> Result<usize, MarketplacePublicCacheError> {
        let mut loaded_entries = 0_usize;
        let mut dir = match fs::read_dir(&self.cache_dir).await {
            Ok(dir) => dir,
            Err(error) => return Err(io_error(error)),
        };

        loop {
            let entry = match dir.next_entry().await {
                Ok(Some(entry)) => entry,
                Ok(None) => return Ok(loaded_entries),
                Err(error) => return Err(io_error(error)),
            };
            let file_type = match entry.file_type().await {
                Ok(file_type) => file_type,
                Err(error) => return Err(io_error(error)),
            };
            if !file_type.is_file() {
                continue;
            }
            let body = match fs::read_to_string(entry.path()).await {
                Ok(body) => body,
                Err(error) => return Err(io_error(error)),
            };
            let disk_entry: DiskCacheEntry = match serde_json::from_str(&body) {
                Ok(disk_entry) => disk_entry,
                Err(error) => return Err(serialization_error(error)),
            };
            if disk_entry.schema_version != DISK_SCHEMA_VERSION {
                continue;
            }
            self.entries.insert(disk_entry.key, disk_entry.value).await;
            loaded_entries += 1;
        }
    }

    async fn read_disk_entry(&self, key: &str) -> Option<String> {
        let path = self.entry_path(key);
        let body = match fs::read_to_string(path).await {
            Ok(body) => body,
            Err(_) => return None,
        };
        let disk_entry: DiskCacheEntry = match serde_json::from_str(&body) {
            Ok(disk_entry) => disk_entry,
            Err(_) => return None,
        };
        if disk_entry.schema_version != DISK_SCHEMA_VERSION {
            return None;
        }
        if disk_entry.key != key {
            return None;
        }
        self.entries
            .insert(disk_entry.key, disk_entry.value.clone())
            .await;
        Some(disk_entry.value)
    }

    async fn write_disk_entry(
        &self,
        key: String,
        value: String,
    ) -> Result<(), MarketplacePublicCacheError> {
        let disk_entry = DiskCacheEntry {
            schema_version: DISK_SCHEMA_VERSION,
            key: key.clone(),
            value,
            stored_at: Utc::now(),
        };
        let body = match serde_json::to_string(&disk_entry) {
            Ok(body) => body,
            Err(error) => return Err(serialization_error(error)),
        };
        let final_path = self.entry_path(&key);
        let temp_path = final_path.with_extension("json.tmp");
        match fs::write(&temp_path, body).await {
            Ok(()) => {}
            Err(error) => return Err(io_error(error)),
        }
        match fs::rename(temp_path, final_path).await {
            Ok(()) => Ok(()),
            Err(error) => Err(io_error(error)),
        }
    }

    fn entry_path(&self, key: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.json", cache_key_hash(key)))
    }
}

fn cache_key_hash(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex_lower(&hasher.finalize())
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn remove_cache_files(cache_dir: PathBuf) -> Result<(), MarketplacePublicCacheError> {
    let entries = match std::fs::read_dir(cache_dir) {
        Ok(entries) => entries,
        Err(error) => return Err(io_error(error)),
    };
    for entry_result in entries {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(error) => return Err(io_error(error)),
        };
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(error) => return Err(io_error(error)),
        };
        if file_type.is_file() {
            match std::fs::remove_file(entry.path()) {
                Ok(()) => {}
                Err(error) => return Err(io_error(error)),
            }
        }
    }
    Ok(())
}
