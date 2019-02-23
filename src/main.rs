use std::time;
use std::thread;


#[derive(Debug)]
enum Tree<T> {
    Debounce,              ## default, no debounce, <specific time>
    Interval(i32),
    UsePolling(bool),
    ThreadPerEvent(bool)
}


fn main() {
    println!("Starting to watch you!");
}
