use std::{
    fmt::Debug,
    ptr::{read_volatile, write_volatile},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread::{sleep, spawn},
    time::Duration,
};

#[allow(non_upper_case_globals)]
static mut threshold: isize = 0;

const MAX_TEST: usize = 100000;
const ELEMENTS_PER_ROW: usize = 6;
const MAX_ROWS: usize = 4;

#[inline(never)]
fn main() {
    let history = native_int();
    pritty_print(history);
    let history = volatile_int();
    pritty_print(history);

    println!("\n\n-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-\n\n");

    counter_race();
    counter_race_atomic();

    println!("\n\n-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-\n\n");

    // called so `cargo asm` can find the function
    index_example();
}

#[inline(never)]
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
#[inline(never)]
pub fn native_int() -> Vec<(isize, usize)> {
    println!("native_int");
    static KEEP_COUNTING: AtomicBool = AtomicBool::new(true);

    let counter = spawn(|| {
        let mut i = 0usize;
        loop {
            //sleep(Duration::from_millis(1));
            // note: mutable statics can be mutated by multiple threads:
            // aliasing violations or data races will cause undefined behavior
            unsafe {
                threshold = (threshold + 1) % 100;
                //println!("counter: {}", threshold);
            }

            // Shutdown code
            if i > 1024 * 1024 {
                i = 0;
                if KEEP_COUNTING.load(Ordering::Relaxed) == false {
                    return;
                }
            } else {
                i += 1;
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
    KEEP_COUNTING.store(false, Ordering::SeqCst);
    counter.join().expect("native_int::counter thread failed");
    history
}

#[inline(never)]
pub fn volatile_int() -> Vec<(isize, usize)> {
    println!("volatile_int");
    static KEEP_COUNTING: AtomicBool = AtomicBool::new(true);

    let counter = spawn(|| {
        let threshold_ptr = unsafe { &mut threshold as *mut isize };
        let mut i = 0usize;

        loop {
            //sleep(Duration::from_millis(1));
            // note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
            unsafe {
                write_volatile(threshold_ptr, (read_volatile(threshold_ptr) + 1) % 100);
                //println!("counter: {}", threshold);
            }

            // Shutdown code
            if i > 1024 * 1024 {
                i = 0;
                if KEEP_COUNTING.load(Ordering::Relaxed) == false {
                    return;
                }
            } else {
                i += 1;
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
    KEEP_COUNTING.store(false, Ordering::SeqCst);
    counter.join().expect("volatile_int::counter thread failed");
    history
}

// -.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-
const N_PARTIES: usize = 4;
const N_INCREMENTS: usize = 100000;

static mut GLOBAL_COUNTER: usize = 0;
#[inline(never)]
pub fn counter_race() {
    (0..N_PARTIES)
        .map(|_i| {
            spawn(move || {
                let counter_ptr = unsafe { &mut GLOBAL_COUNTER as *mut usize };
                for _ in 0..N_INCREMENTS {
                    unsafe {
                        write_volatile(counter_ptr, read_volatile(counter_ptr) + 1);
                    }
                }
                //println!("{} done", _i);
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|t| t.join().expect("counter thread failed"));

    let counter_ptr = unsafe { &mut GLOBAL_COUNTER as *mut usize };
    println!(
        "counter_race(usize)       expected: {}, got: {}",
        N_PARTIES * N_INCREMENTS,
        unsafe { read_volatile(counter_ptr) }
    );
}

static GLOBAL_ATOMIC_COUNTER: AtomicUsize = AtomicUsize::new(0);
#[inline(never)]
pub fn counter_race_atomic() {
    (0..N_PARTIES)
        .map(|_| {
            spawn(|| {
                for _ in 0..N_INCREMENTS {
                    GLOBAL_ATOMIC_COUNTER.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|t| t.join().expect("counter thread failed"));

    println!(
        "counter_race(AtomicUsize) expected: {}, got: {}",
        N_PARTIES * N_INCREMENTS,
        GLOBAL_ATOMIC_COUNTER.load(Ordering::SeqCst)
    );
}

#[inline(never)]
pub fn index_example() {
    let data = vec![42, 42, 42, 42];

    let mut index = 0; // <-- what is the kind of index?
    let length = data.len();
    while index < length {
        println!("{}: {}", index, data[index]);
        index += 1
    }
}
