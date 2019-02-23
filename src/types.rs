use std::time;


//
//
#[derive(Debug)]
struct Config {
    debounce: i32, // default, no debounce, <specific time>
    pollInterval: i32,
    usePolling: bool,
    threadPerEvent: bool
}

// Basic structure describing filesystem even information
//
struct EventInfo {
    path: String,
    time: SystemTime,
    isDirectory: bool,
    description: String
}

// Specific file event reported by a file watcher. Each event contains
// specific information defined in EventInfo
#[derive(Debug)]
enum Event {
    Added(EventInfo),
    Modified(EventInfo),
    Removed(EventInfo),
    WatchDirRemoved(EventInfo),
    Unknown(EventInfo),
}
