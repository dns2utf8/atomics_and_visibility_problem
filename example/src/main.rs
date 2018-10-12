/// This does work with debug but not with release
use std::thread::{sleep, spawn};
use std::time::Duration;

#[allow(non_upper_case_globals)]
static mut threshold: isize = 0;

const MAX_TEST: usize = 100000;

fn main() {
    let _counter = spawn(|| {
        loop {
            //sleep(Duration::from_millis(1));
            // note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
            unsafe {
                threshold = (threshold + 1) % 100;
                //println!("counter: {}", threshold);
            }
        }
    });

    let watcher = spawn(|| {
        sleep(Duration::from_millis(500));
        let mut history = Vec::with_capacity(MAX_TEST);
        let mut last = unsafe { threshold };
        let mut count = 0;
        for _ in 0..MAX_TEST {
            let threshold_local = unsafe { threshold };
            if last == threshold_local {
                count += 1;
            } else {
                history.push((last, count));
                last = threshold_local;
                count = 0;
            }
            //println!("watcher: {}", threshold_local);
        }
        history
    });

    let history = watcher.join().expect("watcher failed");
    println!("{:?}\nn transitions recorded: {}", history, history.len());
    //counter.join();
}
