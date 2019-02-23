use std::time;


// Standard config
//
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Config {
    debounce: i32, // default, no debounce, <specific time>
    pollInterval: i32,
    usePolling: bool,
    threadPerEvent: bool
}

// Basic structure describing filesystem event information
//
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
