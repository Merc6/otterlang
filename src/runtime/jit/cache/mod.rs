// JIT Compilation Cache
pub mod eviction;
pub mod function_cache;
pub mod metadata;

// Re-exports
pub use function_cache::{FunctionCache, CacheStats};
