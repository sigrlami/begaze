use std::time::{SystemTime};

////////////////////////////////////////////////////////////////////////////////

// Manager configuration to collectively listen
// file changes with similar behaviour
#[derive( PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct WatchManager {
    pub config: WatchConfig,
    pub listener: u32
}


// Standard config
//
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Config {
    pub debounce: i32, // default, no debounce, <specific time>
    pub pollInterval: i32,
    pub usePolling: bool,
    pub threadPerEvent: bool
}

// initialize default structure for config
//
impl Default for Config {
    fn default() -> Config {
        Config {
            debounce: 0,
            pollInterval: 10,
            usePolling: false,
            threadPerEvent: false,
        }
    }
}

// Basic structure describing filesystem event information
//
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct EventInfo {
    path: String,
    time: SystemTime,
    isDirectory: bool,
    description: String
}

// Specific file event reported by a file watcher. Each event contains
// specific information defined in EventInfo
#[derive(Debug)]
pub enum Event {
    Added(EventInfo),
    Modified(EventInfo),
    Removed(EventInfo),
    WatchDirRemoved(EventInfo),
    Unknown(EventInfo),
}
