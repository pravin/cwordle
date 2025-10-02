// Library exports for testing and reuse

pub mod config;
pub mod gamestate;
pub mod stats;
pub mod words;

// Re-export commonly used types
pub use config::Config;
pub use gamestate::{GameData, GameStatus, KeyBoardHelperState};
pub use stats::{GameStats, StatsManager};
