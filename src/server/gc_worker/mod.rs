// Copyright 2018 TiKV Project Authors. Licensed under Apache-2.0.

mod applied_lock_collector;
mod compaction_filter;
mod config;
mod gc_manager;
mod gc_worker;

// TODO: Use separated error type for GCWorker instead.
pub use crate::storage::{Callback, Error, ErrorInner, Result};
pub use compaction_filter::WriteCompactionFilterFactory;
use compaction_filter::{init_compaction_filter, is_compaction_filter_allowd};
pub use config::{GcConfig, GcWorkerConfigManager, DEFAULT_GC_BATCH_KEYS};
pub use gc_manager::AutoGcConfig;
pub use gc_worker::{GcSafePointProvider, GcTask, GcWorker, GC_MAX_EXECUTING_TASKS};

#[cfg(test)]
pub use compaction_filter::tests::gc_by_compact;
