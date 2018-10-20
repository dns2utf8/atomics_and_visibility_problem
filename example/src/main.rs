use std::thread::{sleep, spawn};
use std::time::Duration;
use std::ptr::{read_volatile, write_volatile};
use std::fmt::Debug;

#[allow(non_upper_case_globals)]
static mut threshold: isize = 0;

const MAX_TEST: usize = 100000;
const ELEMENTS_PER_ROW: usize = 6;
const MAX_ROWS: usize = 4;

fn main() {
    let history = native_int();
    pritty_print(history);
    let history = volatile_int();
    pritty_print(history);
}

fn pritty_print<T: Debug>(list: Vec<T>) {
    const MAX: usize = ELEMENTS_PER_ROW * MAX_ROWS;

    print!("[");
    if list.len() > 0 {
        let mut iter = list.iter();
        let mut skipper;
        let iter: &mut Iterator<Item = _> = if list.len() > MAX {
            print!("\n    ...");
            skipper = iter.skip(list.len() - MAX);
            &mut skipper
        } else {
            &mut iter
        };

        let mut i = 0;
        let mut s = String::with_capacity(MAX * 10);
        for el in iter {
            if i % ELEMENTS_PER_ROW == 0 {
                s += "\n    ";
            }
            s.push_str(&format!("{:?}, ", el));
            i += 1;
        }
        s.pop();
        s.pop();
        println!("{}", s);
    }

    println!("]\nn transitions recorded: {}\n", list.len());
}

/// This does work with debug but not with release
pub fn native_int() -> Vec<(isize, usize)> {
    println!("native_int");
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
    //counter.join();
    history
}

pub fn volatile_int() -> Vec<(isize, usize)> {
    println!("volatile_int");
    let _counter = spawn(|| {
        let threshold_ptr = unsafe { &mut threshold as *mut isize };
        loop {
            //sleep(Duration::from_millis(1));
            // note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
            unsafe {
                write_volatile(threshold_ptr, (read_volatile(threshold_ptr) + 1) % 100);
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

    watcher.join().expect("watcher failed")
}
