// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/hazelcast/hazelcast-platform-operator/hazelcast.com/v1alpha1/maps.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// MapSpec defines the desired state of Hazelcast Map Config
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "hazelcast.com", version = "v1alpha1", kind = "Map", plural = "maps")]
#[kube(namespaced)]
#[kube(status = "MapStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MapSpec {
    /// Number of asynchronous backups.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "asyncBackupCount")]
    pub async_backup_count: Option<i32>,
    /// Attributes to be used with Predicates API. You can learn more at https://docs.hazelcast.com/hazelcast/latest/query/predicate-overview#creating-custom-query-attributes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<MapAttributes>>,
    /// Number of synchronous backups.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupCount")]
    pub backup_count: Option<i32>,
    /// EntryListeners contains the configuration for the map-level or entry-based events listeners provided by the Hazelcast’s eventing framework. You can learn more at https://docs.hazelcast.com/hazelcast/latest/events/object-events.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "entryListeners")]
    pub entry_listeners: Option<Vec<MapEntryListeners>>,
    /// EventJournal specifies event journal configuration of the Map
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eventJournal")]
    pub event_journal: Option<MapEventJournal>,
    /// Configuration for removing data from the map when it reaches its max size. It can be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eviction: Option<MapEviction>,
    /// HazelcastResourceName defines the name of the Hazelcast resource that this resource is created for.
    #[serde(rename = "hazelcastResourceName")]
    pub hazelcast_resource_name: String,
    /// InMemoryFormat specifies in which format data will be stored in your map
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inMemoryFormat")]
    pub in_memory_format: Option<MapInMemoryFormat>,
    /// Indexes to be created for the map data. You can learn more at https://docs.hazelcast.com/hazelcast/latest/query/indexing-maps. It cannot be updated after map config is created successfully.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexes: Option<Vec<MapIndexes>>,
    /// Configuration options when you want to load/store the map entries from/to a persistent data store such as a relational database You can learn more at https://docs.hazelcast.com/hazelcast/latest/data-structures/working-with-external-data
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mapStore")]
    pub map_store: Option<MapMapStore>,
    /// Maximum time in seconds for each entry to stay idle in the map. Entries that are idle for more than this time are evicted automatically. It can be updated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxIdleSeconds")]
    pub max_idle_seconds: Option<i32>,
    /// MerkleTree defines the configuration for the Merkle tree data structure.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "merkleTree")]
    pub merkle_tree: Option<MapMerkleTree>,
    /// Name of the data structure config to be created. If empty, CR name will be used. It cannot be updated after the config is created successfully.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// InMemoryFormat specifies near cache configuration for map
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nearCache")]
    pub near_cache: Option<MapNearCache>,
    /// When enabled, map data will be persisted. It cannot be updated after map config is created successfully.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "persistenceEnabled")]
    pub persistence_enabled: Option<bool>,
    /// TieredStore enables the Hazelcast's Tiered-Store feature for the Map
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tieredStore")]
    pub tiered_store: Option<MapTieredStore>,
    /// Maximum time in seconds for each entry to stay in the map. If it is not 0, entries that are older than this time and not updated for this time are evicted automatically. It can be updated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeToLiveSeconds")]
    pub time_to_live_seconds: Option<i32>,
    /// Name of the User Code Namespace applied to this instance
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userCodeNamespace")]
    pub user_code_namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MapAttributes {
    /// Name of the extractor class https://docs.hazelcast.com/hazelcast/latest/query/predicate-overview#implementing-a-valueextractor
    #[serde(rename = "extractorClassName")]
    pub extractor_class_name: String,
    /// Name of the attribute https://docs.hazelcast.com/hazelcast/latest/query/predicate-overview#creating-custom-query-attributes
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MapEntryListeners {
    /// ClassName is the fully qualified name of the class that implements any of the Listener interface.
    #[serde(rename = "className")]
    pub class_name: String,
    /// IncludeValues is an optional attribute that indicates whether the event will contain the map value. Defaults to true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "includeValues")]
    pub include_values: Option<bool>,
    /// Local is an optional attribute that indicates whether the map on the local member can be listened to. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<bool>,
}

/// EventJournal specifies event journal configuration of the Map
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MapEventJournal {
    /// Capacity sets the capacity of the ringbuffer underlying the event journal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    /// TimeToLiveSeconds indicates how long the items remain in the event journal before they are expired.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeToLiveSeconds")]
    pub time_to_live_seconds: Option<i32>,
}

/// Configuration for removing data from the map when it reaches its max size. It can be updated.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MapEviction {
    /// Eviction policy to be applied when map reaches its max size according to the max size policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionPolicy")]
    pub eviction_policy: Option<MapEvictionEvictionPolicy>,
    /// Max size of the map.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSize")]
    pub max_size: Option<i32>,
    /// Policy for deciding if the maxSize is reached.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSizePolicy")]
    pub max_size_policy: Option<MapEvictionMaxSizePolicy>,
}

/// Configuration for removing data from the map when it reaches its max size. It can be updated.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MapEvictionEvictionPolicy {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "LRU")]
    Lru,
    #[serde(rename = "LFU")]
    Lfu,
    #[serde(rename = "RANDOM")]
    Random,
}

/// Configuration for removing data from the map when it reaches its max size. It can be updated.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MapEvictionMaxSizePolicy {
    #[serde(rename = "PER_NODE")]
    PerNode,
    #[serde(rename = "PER_PARTITION")]
    PerPartition,
    #[serde(rename = "USED_HEAP_SIZE")]
    UsedHeapSize,
    #[serde(rename = "USED_HEAP_PERCENTAGE")]
    UsedHeapPercentage,
    #[serde(rename = "FREE_HEAP_SIZE")]
    FreeHeapSize,
    #[serde(rename = "FREE_HEAP_PERCENTAGE")]
    FreeHeapPercentage,
    #[serde(rename = "USED_NATIVE_MEMORY_SIZE")]
    UsedNativeMemorySize,
    #[serde(rename = "USED_NATIVE_MEMORY_PERCENTAGE")]
    UsedNativeMemoryPercentage,
    #[serde(rename = "FREE_NATIVE_MEMORY_SIZE")]
    FreeNativeMemorySize,
    #[serde(rename = "FREE_NATIVE_MEMORY_PERCENTAGE")]
    FreeNativeMemoryPercentage,
    #[serde(rename = "ENTRY_COUNT")]
    EntryCount,
}

/// MapSpec defines the desired state of Hazelcast Map Config
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MapInMemoryFormat {
    #[serde(rename = "BINARY")]
    Binary,
    #[serde(rename = "OBJECT")]
    Object,
    #[serde(rename = "NATIVE")]
    Native,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MapIndexes {
    /// Attributes of the index.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
    /// Options for "BITMAP" index type. See https://docs.hazelcast.com/hazelcast/latest/query/indexing-maps#configuring-bitmap-indexes
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bitMapIndexOptions")]
    pub bit_map_index_options: Option<MapIndexesBitMapIndexOptions>,
    /// Name of the index config.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Type of the index. See https://docs.hazelcast.com/hazelcast/latest/query/indexing-maps#index-types
    #[serde(rename = "type")]
    pub r#type: MapIndexesType,
}

/// Options for "BITMAP" index type. See https://docs.hazelcast.com/hazelcast/latest/query/indexing-maps#configuring-bitmap-indexes
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MapIndexesBitMapIndexOptions {
    #[serde(rename = "uniqueKey")]
    pub unique_key: String,
    #[serde(rename = "uniqueKeyTransition")]
    pub unique_key_transition: MapIndexesBitMapIndexOptionsUniqueKeyTransition,
}

/// Options for "BITMAP" index type. See https://docs.hazelcast.com/hazelcast/latest/query/indexing-maps#configuring-bitmap-indexes
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MapIndexesBitMapIndexOptionsUniqueKeyTransition {
    #[serde(rename = "OBJECT")]
    Object,
    #[serde(rename = "LONG")]
    Long,
    #[serde(rename = "RAW")]
    Raw,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MapIndexesType {
    #[serde(rename = "SORTED")]
    Sorted,
    #[serde(rename = "HASH")]
    Hash,
    #[serde(rename = "BITMAP")]
    Bitmap,
}

/// Configuration options when you want to load/store the map entries from/to a persistent data store such as a relational database You can learn more at https://docs.hazelcast.com/hazelcast/latest/data-structures/working-with-external-data
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MapMapStore {
    /// Name of your class implementing MapLoader and/or MapStore interface.
    #[serde(rename = "className")]
    pub class_name: String,
    /// Sets the initial entry loading mode.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initialMode")]
    pub initial_mode: Option<MapMapStoreInitialMode>,
    /// Properties can be used for giving information to the MapStore implementation
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "propertiesSecretName")]
    pub properties_secret_name: Option<String>,
    /// Used to create batches when writing to map store.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "writeBatchSize")]
    pub write_batch_size: Option<i32>,
    /// It is meaningful if you are using write behind in MapStore. When it is set to true, only the latest store operation on a key during the write-delay-seconds will be reflected to MapStore.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "writeCoalescing")]
    pub write_coalescing: Option<bool>,
    /// Number of seconds to delay the storing of entries.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "writeDelaySeconds")]
    pub write_delay_seconds: Option<i32>,
}

/// Configuration options when you want to load/store the map entries from/to a persistent data store such as a relational database You can learn more at https://docs.hazelcast.com/hazelcast/latest/data-structures/working-with-external-data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MapMapStoreInitialMode {
    #[serde(rename = "LAZY")]
    Lazy,
    #[serde(rename = "EAGER")]
    Eager,
}

/// MerkleTree defines the configuration for the Merkle tree data structure.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MapMerkleTree {
    /// Depth of the merkle tree.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
}

/// InMemoryFormat specifies near cache configuration for map
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MapNearCache {
    /// CacheLocalEntries specifies whether the local entries are cached
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheLocalEntries")]
    pub cache_local_entries: Option<bool>,
    /// NearCacheEviction specifies the eviction behavior in Near Cache
    pub eviction: MapNearCacheEviction,
    /// InMemoryFormat specifies in which format data will be stored in your near cache
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inMemoryFormat")]
    pub in_memory_format: Option<MapNearCacheInMemoryFormat>,
    /// InvalidateOnChange specifies whether the cached entries are evicted when the entries are updated or removed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "invalidateOnChange")]
    pub invalidate_on_change: Option<bool>,
    /// MaxIdleSeconds Maximum number of seconds each entry can stay in the Near Cache as untouched (not read)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxIdleSeconds")]
    pub max_idle_seconds: Option<i64>,
    /// Name is name of the near cache
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// TimeToLiveSeconds maximum number of seconds for each entry to stay in the Near Cache
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeToLiveSeconds")]
    pub time_to_live_seconds: Option<i64>,
}

/// NearCacheEviction specifies the eviction behavior in Near Cache
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MapNearCacheEviction {
    /// EvictionPolicy to be applied when near cache reaches its max size according to the max size policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionPolicy")]
    pub eviction_policy: Option<MapNearCacheEvictionEvictionPolicy>,
    /// MaxSizePolicy for deciding if the maxSize is reached.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSizePolicy")]
    pub max_size_policy: Option<MapNearCacheEvictionMaxSizePolicy>,
    /// Size is maximum size of the Near Cache used for max-size-policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

/// NearCacheEviction specifies the eviction behavior in Near Cache
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MapNearCacheEvictionEvictionPolicy {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "LRU")]
    Lru,
    #[serde(rename = "LFU")]
    Lfu,
    #[serde(rename = "RANDOM")]
    Random,
}

/// NearCacheEviction specifies the eviction behavior in Near Cache
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MapNearCacheEvictionMaxSizePolicy {
    #[serde(rename = "PER_NODE")]
    PerNode,
    #[serde(rename = "PER_PARTITION")]
    PerPartition,
    #[serde(rename = "USED_HEAP_SIZE")]
    UsedHeapSize,
    #[serde(rename = "USED_HEAP_PERCENTAGE")]
    UsedHeapPercentage,
    #[serde(rename = "FREE_HEAP_SIZE")]
    FreeHeapSize,
    #[serde(rename = "FREE_HEAP_PERCENTAGE")]
    FreeHeapPercentage,
    #[serde(rename = "USED_NATIVE_MEMORY_SIZE")]
    UsedNativeMemorySize,
    #[serde(rename = "USED_NATIVE_MEMORY_PERCENTAGE")]
    UsedNativeMemoryPercentage,
    #[serde(rename = "FREE_NATIVE_MEMORY_SIZE")]
    FreeNativeMemorySize,
    #[serde(rename = "FREE_NATIVE_MEMORY_PERCENTAGE")]
    FreeNativeMemoryPercentage,
    #[serde(rename = "ENTRY_COUNT")]
    EntryCount,
}

/// InMemoryFormat specifies near cache configuration for map
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MapNearCacheInMemoryFormat {
    #[serde(rename = "BINARY")]
    Binary,
    #[serde(rename = "OBJECT")]
    Object,
    #[serde(rename = "NATIVE")]
    Native,
}

/// TieredStore enables the Hazelcast's Tiered-Store feature for the Map
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MapTieredStore {
    /// diskDeviceName defines the name of the device for a given disk tier.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskDeviceName")]
    pub disk_device_name: Option<String>,
    /// MemoryCapacity sets Memory tier capacity, i.e., how much main memory should this tier consume at most.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memoryCapacity")]
    pub memory_capacity: Option<IntOrString>,
}

/// MapStatus defines the observed state of Map
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MapStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memberStatuses")]
    pub member_statuses: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<MapStatusState>,
}

/// MapStatus defines the observed state of Map
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MapStatusState {
    Success,
    Failed,
    Pending,
    Persisting,
    Terminating,
}

