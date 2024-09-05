pub use lighthouse_metrics::{set_gauge, try_create_int_gauge, *};

use directory::size_of_dir;
use std::path::Path;
use std::sync::LazyLock;

/*
 * General
 */
pub static DISK_DB_SIZE: LazyLock<Result<IntGauge>> = LazyLock::new(|| {
    try_create_int_gauge(
        "store_disk_db_size",
        "Size of the hot on-disk database (bytes)",
    )
});
pub static FREEZER_DB_SIZE: LazyLock<Result<IntGauge>> = LazyLock::new(|| {
    try_create_int_gauge(
        "store_freezer_db_size",
        "Size of the on-disk freezer database (bytes)",
    )
});
pub static DISK_DB_WRITE_BYTES: LazyLock<Result<IntCounterVec>> = LazyLock::new(|| {
    try_create_int_counter_vec(
        "store_disk_db_write_bytes_total",
        "Number of bytes attempted to be written to the hot on-disk DB",
        &["col"],
    )
});
pub static DISK_DB_READ_BYTES: LazyLock<Result<IntCounterVec>> = LazyLock::new(|| {
    try_create_int_counter_vec(
        "store_disk_db_read_bytes_total",
        "Number of bytes read from the hot on-disk DB",
        &["col"],
    )
});
pub static DISK_DB_READ_COUNT: LazyLock<Result<IntCounterVec>> = LazyLock::new(|| {
    try_create_int_counter_vec(
        "store_disk_db_read_count_total",
        "Total number of reads to the hot on-disk DB",
        &["col"],
    )
});
pub static DISK_DB_WRITE_COUNT: LazyLock<Result<IntCounterVec>> = LazyLock::new(|| {
    try_create_int_counter_vec(
        "store_disk_db_write_count_total",
        "Total number of writes to the hot on-disk DB",
        &["col"],
    )
});
pub static DISK_DB_READ_TIMES: LazyLock<Result<Histogram>> = LazyLock::new(|| {
    try_create_histogram(
        "store_disk_db_read_seconds",
        "Time taken to write bytes to store.",
    )
});
pub static DISK_DB_WRITE_TIMES: LazyLock<Result<Histogram>> = LazyLock::new(|| {
    try_create_histogram(
        "store_disk_db_write_seconds",
        "Time taken to write bytes to store.",
    )
});
pub static DISK_DB_EXISTS_COUNT: LazyLock<Result<IntCounterVec>> = LazyLock::new(|| {
    try_create_int_counter_vec(
        "store_disk_db_exists_count_total",
        "Total number of checks if a key is in the hot on-disk DB",
        &["col"],
    )
});
pub static DISK_DB_DELETE_COUNT: LazyLock<Result<IntCounterVec>> = LazyLock::new(|| {
    try_create_int_counter_vec(
        "store_disk_db_delete_count_total",
        "Total number of deletions from the hot on-disk DB",
        &["col"],
    )
});
/*
 * Beacon State
 */
pub static BEACON_STATE_GET_COUNT: LazyLock<Result<IntCounter>> = LazyLock::new(|| {
    try_create_int_counter(
        "store_beacon_state_get_total",
        "Total number of beacon states requested from the store (cache or DB)",
    )
});
pub static BEACON_STATE_HOT_GET_COUNT: LazyLock<Result<IntCounter>> = LazyLock::new(|| {
    try_create_int_counter(
        "store_beacon_state_hot_get_total",
        "Total number of hot beacon states requested from the store (cache or DB)",
    )
});
pub static BEACON_STATE_READ_TIMES: LazyLock<Result<Histogram>> = LazyLock::new(|| {
    try_create_histogram(
        "store_beacon_state_read_seconds",
        "Total time required to read a BeaconState from the database",
    )
});
pub static BEACON_STATE_READ_OVERHEAD_TIMES: LazyLock<Result<Histogram>> = LazyLock::new(|| {
    try_create_histogram(
        "store_beacon_state_read_overhead_seconds",
        "Overhead on reading a beacon state from the DB (e.g., decoding)",
    )
});
pub static BEACON_STATE_READ_COUNT: LazyLock<Result<IntCounter>> = LazyLock::new(|| {
    try_create_int_counter(
        "store_beacon_state_read_total",
        "Total number of beacon state reads from the DB",
    )
});
pub static BEACON_STATE_READ_BYTES: LazyLock<Result<IntCounter>> = LazyLock::new(|| {
    try_create_int_counter(
        "store_beacon_state_read_bytes_total",
        "Total number of beacon state bytes read from the DB",
    )
});
pub static BEACON_STATE_WRITE_OVERHEAD_TIMES: LazyLock<Result<Histogram>> = LazyLock::new(|| {
    try_create_histogram(
        "store_beacon_state_write_overhead_seconds",
        "Overhead on writing a beacon state to the DB (e.g., encoding)",
    )
});
pub static BEACON_STATE_WRITE_COUNT: LazyLock<Result<IntCounter>> = LazyLock::new(|| {
    try_create_int_counter(
        "store_beacon_state_write_total",
        "Total number of beacon state writes the DB",
    )
});
pub static BEACON_STATE_WRITE_BYTES: LazyLock<Result<IntCounter>> = LazyLock::new(|| {
    try_create_int_counter(
        "store_beacon_state_write_bytes_total",
        "Total number of beacon state bytes written to the DB",
    )
});
/*
 * Beacon Block
 */
pub static BEACON_BLOCK_GET_COUNT: LazyLock<Result<IntCounter>> = LazyLock::new(|| {
    try_create_int_counter(
        "store_beacon_block_get_total",
        "Total number of beacon blocks requested from the store (cache or DB)",
    )
});
pub static BEACON_BLOCK_CACHE_HIT_COUNT: LazyLock<Result<IntCounter>> = LazyLock::new(|| {
    try_create_int_counter(
        "store_beacon_block_cache_hit_total",
        "Number of hits to the store's block cache",
    )
});

/*
 * Caches
 */
pub static BEACON_BLOBS_CACHE_HIT_COUNT: LazyLock<Result<IntCounter>> = LazyLock::new(|| {
    try_create_int_counter(
        "store_beacon_blobs_cache_hit_total",
        "Number of hits to the store's blob cache",
    )
});
pub static STORE_BEACON_BLOCK_CACHE_SIZE: LazyLock<Result<IntGauge>> = LazyLock::new(|| {
    try_create_int_gauge(
        "store_beacon_block_cache_size",
        "Current count of items in beacon store block cache",
    )
});
pub static STORE_BEACON_BLOB_CACHE_SIZE: LazyLock<Result<IntGauge>> = LazyLock::new(|| {
    try_create_int_gauge(
        "store_beacon_blob_cache_size",
        "Current count of items in beacon store blob cache",
    )
});
pub static STORE_BEACON_STATE_CACHE_SIZE: LazyLock<Result<IntGauge>> = LazyLock::new(|| {
    try_create_int_gauge(
        "store_beacon_state_cache_size",
        "Current count of items in beacon store state cache",
    )
});
pub static STORE_BEACON_HISTORIC_STATE_CACHE_SIZE: LazyLock<Result<IntGauge>> =
    LazyLock::new(|| {
        try_create_int_gauge(
            "store_beacon_historic_state_cache_size",
            "Current count of items in beacon store historic state cache",
        )
    });
pub static STORE_BEACON_HDIFF_BUFFER_CACHE_SIZE: LazyLock<Result<IntGauge>> = LazyLock::new(|| {
    try_create_int_gauge(
        "store_beacon_hdiff_buffer_cache_size",
        "Current count of items in beacon store hdiff buffer cache",
    )
});
pub static STORE_BEACON_HDIFF_BUFFER_CACHE_BYTE_SIZE: LazyLock<Result<IntGauge>> =
    LazyLock::new(|| {
        try_create_int_gauge(
            "store_beacon_hdiff_buffer_cache_byte_size",
            "Current byte size sum of all elements in beacon store hdiff buffer cache",
        )
    });
pub static STORE_BEACON_STATE_FREEZER_COMPRESS_TIME: LazyLock<Result<Histogram>> =
    LazyLock::new(|| {
        try_create_histogram(
            "store_beacon_state_compress_seconds",
            "Time taken to compress a state snapshot for the freezer DB",
        )
    });
pub static STORE_BEACON_STATE_FREEZER_DECOMPRESS_TIME: LazyLock<Result<Histogram>> =
    LazyLock::new(|| {
        try_create_histogram(
            "store_beacon_state_decompress_seconds",
            "Time taken to decompress a state snapshot for the freezer DB",
        )
    });
pub static STORE_BEACON_HDIFF_BUFFER_APPLY_TIME: LazyLock<Result<Histogram>> =
    LazyLock::new(|| {
        try_create_histogram(
            "store_beacon_hdiff_buffer_apply_seconds",
            "Time taken to apply hdiff buffer to a state buffer",
        )
    });
pub static STORE_BEACON_HDIFF_BUFFER_COMPUTE_TIME: LazyLock<Result<Histogram>> =
    LazyLock::new(|| {
        try_create_histogram(
            "store_beacon_hdiff_buffer_compute_seconds",
            "Time taken to compute hdiff buffer to a state buffer",
        )
    });
pub static STORE_BEACON_HDIFF_BUFFER_LOAD_TIME: LazyLock<Result<Histogram>> = LazyLock::new(|| {
    try_create_histogram(
        "store_beacon_hdiff_buffer_load_seconds",
        "Time taken to load an hdiff buffer",
    )
});
pub static STORE_BEACON_HDIFF_BUFFER_CACHE_HIT: LazyLock<Result<IntCounter>> =
    LazyLock::new(|| {
        try_create_int_counter(
            "store_beacon_hdiff_buffer_cache_hit_total",
            "Total count of hdiff buffer cache hits",
        )
    });
pub static STORE_BEACON_HDIFF_BUFFER_CACHE_MISS: LazyLock<Result<IntCounter>> =
    LazyLock::new(|| {
        try_create_int_counter(
            "store_beacon_hdiff_buffer_cache_miss_total",
            "Total count of hdiff buffer cache miss",
        )
    });
pub static STORE_BEACON_REPLAYED_BLOCKS: LazyLock<Result<IntCounter>> = LazyLock::new(|| {
    try_create_int_counter(
        "store_beacon_replayed_blocks_total",
        "Total count of replayed blocks",
    )
});
pub static BEACON_DATA_COLUMNS_CACHE_HIT_COUNT: LazyLock<Result<IntCounter>> =
    LazyLock::new(|| {
        try_create_int_counter(
            "store_beacon_data_columns_cache_hit_total",
            "Number of hits to the store's data column cache",
        )
    });

/// Updates the global metrics registry with store-related information.
pub fn scrape_for_metrics(db_path: &Path, freezer_db_path: &Path) {
    let db_size = size_of_dir(db_path);
    set_gauge(&DISK_DB_SIZE, db_size as i64);
    let freezer_db_size = size_of_dir(freezer_db_path);
    set_gauge(&FREEZER_DB_SIZE, freezer_db_size as i64);
}
