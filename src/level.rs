#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Exception severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Level {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
    Fatal = 4,
}

impl Default for Level {
    fn default() -> Self {
        Level::Error
    }
}
