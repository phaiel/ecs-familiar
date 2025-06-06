use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tokio::time::{interval, Duration};

// Note: These will be used when full persistence integration is implemented
#[allow(unused_imports)]
use crate::components::{MemoryLayer, MemoryLayerType, ConsolidationStatus};

/// Persistence layer that manages working memory (Redis) and primary memory (DB)
pub struct MemoryPersistence {
    // Redis client would go here in real implementation
    // redis_client: redis::Client,
    
    // ChronicleDB client would go here in real implementation
    // chronicle_client: ChronicleClient,
    
    // Local cache for working memory
    working_memory: HashMap<Uuid, WorkingMemoryEntry>,
    
    // Consolidation queue
    consolidation_queue: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingMemoryEntry {
    pub entity_id: Uuid,
    pub data: Vec<u8>, // Serialized entity data
    pub access_count: u32,
    pub last_accessed: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub ttl_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryMemoryEntry {
    pub entity_id: Uuid,
    pub version: u64,
    pub data: Vec<u8>,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    EntityCreated,
    ComponentUpdated,
    EntityTagAdded,
    EntitySoftDeleted,
    EntityConsolidated,
}

impl MemoryPersistence {
    pub fn new() -> Self {
        Self {
            working_memory: HashMap::new(),
            consolidation_queue: Vec::new(),
        }
    }

    /// Store entity in working memory (Redis in real implementation)
    pub async fn store_working_memory(&mut self, entity_id: Uuid, data: Vec<u8>, ttl_seconds: Option<u64>) -> Result<(), PersistenceError> {
        let entry = WorkingMemoryEntry {
            entity_id,
            data,
            access_count: 1,
            last_accessed: Utc::now(),
            created_at: Utc::now(),
            ttl_seconds,
        };

        self.working_memory.insert(entity_id, entry);
        
        // TODO: Redis implementation
        // self.redis_client.set_ex(entity_id.to_string(), data, ttl_seconds.unwrap_or(3600)).await?;
        
        Ok(())
    }

    /// Retrieve entity from working memory with access tracking
    pub async fn get_working_memory(&mut self, entity_id: Uuid) -> Result<Option<Vec<u8>>, PersistenceError> {
        if let Some(entry) = self.working_memory.get_mut(&entity_id) {
            entry.access_count += 1;
            entry.last_accessed = Utc::now();
            Ok(Some(entry.data.clone()))
        } else {
            // TODO: Redis implementation
            // let data = self.redis_client.get(entity_id.to_string()).await?;
            Ok(None)
        }
    }

    /// Append-only write to primary memory (ChronicleDB in real implementation)
    pub async fn append_primary_memory(&self, entry: PrimaryMemoryEntry) -> Result<(), PersistenceError> {
        // TODO: ChronicleDB implementation
        // Event sourcing: all changes are appended, never updated
        println!("📚 Appending to primary memory: {:?} v{}", entry.entity_id, entry.version);
        Ok(())
    }

    /// Queue entity for consolidation from working to primary memory
    pub fn queue_for_consolidation(&mut self, entity_id: Uuid) {
        if !self.consolidation_queue.contains(&entity_id) {
            self.consolidation_queue.push(entity_id);
        }
    }

    /// Process consolidation queue - move entities from working to primary memory
    pub async fn process_consolidation(&mut self) -> Result<usize, PersistenceError> {
        let mut processed = 0;
        let entity_ids: Vec<Uuid> = self.consolidation_queue.drain(..).collect();

        for entity_id in entity_ids {
            if let Some(working_entry) = self.working_memory.get(&entity_id) {
                let data = working_entry.data.clone();
                let checksum = self.calculate_checksum(&data);
                
                let primary_entry = PrimaryMemoryEntry {
                    entity_id,
                    version: 1, // TODO: Get actual version from entity
                    data,
                    timestamp: Utc::now(),
                    event_type: EventType::EntityConsolidated,
                    checksum,
                };

                self.append_primary_memory(primary_entry).await?;
                processed += 1;
            }
        }

        Ok(processed)
    }

    /// Clean up expired working memory entries
    pub async fn cleanup_working_memory(&mut self) -> Result<usize, PersistenceError> {
        let now = Utc::now();
        let mut removed = 0;

        self.working_memory.retain(|_id, entry| {
            if let Some(ttl) = entry.ttl_seconds {
                let age = now.signed_duration_since(entry.created_at);
                if age.num_seconds() > ttl as i64 {
                    removed += 1;
                    false
                } else {
                    true
                }
            } else {
                true
            }
        });

        Ok(removed)
    }

    /// Get memory statistics
    pub fn get_memory_stats(&self) -> MemoryStats {
        MemoryStats {
            working_memory_entries: self.working_memory.len(),
            consolidation_queue_size: self.consolidation_queue.len(),
            total_access_count: self.working_memory.values().map(|e| e.access_count).sum(),
        }
    }

    fn calculate_checksum(&self, data: &[u8]) -> String {
        // Simple checksum for now - real implementation would use proper hashing
        format!("{:x}", data.len())
    }

    /// Start background consolidation process
    pub async fn start_background_consolidation(&mut self) {
        let mut consolidation_interval = interval(Duration::from_secs(30));
        let mut cleanup_interval = interval(Duration::from_secs(300)); // 5 minutes

        loop {
            tokio::select! {
                _ = consolidation_interval.tick() => {
                    if let Ok(processed) = self.process_consolidation().await {
                        if processed > 0 {
                            println!("🔄 Consolidated {} entities to primary memory", processed);
                        }
                    }
                }
                _ = cleanup_interval.tick() => {
                    if let Ok(cleaned) = self.cleanup_working_memory().await {
                        if cleaned > 0 {
                            println!("🧹 Cleaned up {} expired working memory entries", cleaned);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub working_memory_entries: usize,
    pub consolidation_queue_size: usize,
    pub total_access_count: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("Redis error: {0}")]
    Redis(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Entity not found: {0}")]
    EntityNotFound(Uuid),
} 