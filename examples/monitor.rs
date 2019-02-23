use begaze::{Watcher};
use std::path::Path;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    println!("watching {}", path);
    //if let Err(e) = watch(path) {
    //    println!("error: {:?}", e)
    //}
}
