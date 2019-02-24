use begaze::{WatchManager};
use std::path::Path;

#[test]
fn test_watch() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    println!("watching {}", path);
}
