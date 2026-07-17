pub mod strategy;
pub mod summary;

pub use strategy::{analyze_live_strategy, StrategyInsight, StrategySeverity};
pub use summary::snapshot_context;
