#![feature(test)]

extern crate test;

use std::mem::size_of;
use std::mem::transmute;
use std::sync::atomic::{AtomicUsize, Ordering};
use test::{black_box, Bencher};

// 64 Byte <- size of cache line
#[repr(align(64))]
struct Align64<T>(T);

trait Atom {
    fn new() -> Self;

    fn get_ref(&self, ith: usize) -> &AtomicUsize;
}

/*impl Atom for AtomicUsize {
    fn new() -> Self {
        AtomicUsize::new(0)
    }
}*/

#[repr(align(64))]
struct CacheLineAware {
    a: Align64<AtomicUsize>,
    b: Align64<AtomicUsize>,
    c: Align64<AtomicUsize>,
    d: Align64<AtomicUsize>,
    e: Align64<AtomicUsize>,
    f: Align64<AtomicUsize>,
    g: Align64<AtomicUsize>,
    h: Align64<AtomicUsize>,
    //c_d: Align64<(AtomicUsize, AtomicUsize)>,
}
impl Atom for CacheLineAware {
    fn new() -> Self {
        CacheLineAware {
            a: Align64(AtomicUsize::new(0)),
            b: Align64(AtomicUsize::new(0)),
            c: Align64(AtomicUsize::new(0)),
            d: Align64(AtomicUsize::new(0)),
            e: Align64(AtomicUsize::new(0)),
            f: Align64(AtomicUsize::new(0)),
            g: Align64(AtomicUsize::new(0)),
            h: Align64(AtomicUsize::new(0)),
        }
    }
    fn get_ref(&self, ith: usize) -> &AtomicUsize {
        match ith {
            0 => &self.a.0,
            1 => &self.b.0,
            2 => &self.c.0,
            3 => &self.d.0,
            4 => &self.e.0,
            5 => &self.f.0,
            6 => &self.g.0,
            7 => &self.h.0,
            _ => unreachable!(),
        }
    }
}

#[repr(align(64))]
struct NormalSized {
    a: AtomicUsize,
    b: AtomicUsize,
    c: AtomicUsize,
    d: AtomicUsize,
    e: AtomicUsize,
    f: AtomicUsize,
    g: AtomicUsize,
    h: AtomicUsize,
}
impl Atom for NormalSized {
    fn new() -> Self {
        NormalSized {
            a: AtomicUsize::new(0),
            b: AtomicUsize::new(0),
            c: AtomicUsize::new(0),
            d: AtomicUsize::new(0),
            e: AtomicUsize::new(0),
            f: AtomicUsize::new(0),
            g: AtomicUsize::new(0),
            h: AtomicUsize::new(0),
        }
    }
    fn get_ref(&self, ith: usize) -> &AtomicUsize {
        match ith {
            0 => &self.a,
            1 => &self.b,
            2 => &self.c,
            3 => &self.d,
            4 => &self.e,
            5 => &self.f,
            6 => &self.g,
            7 => &self.h,
            _ => unreachable!(),
        }
    }
}

struct Normal {
    a: AtomicUsize,
    b: AtomicUsize,
    c: AtomicUsize,
    d: AtomicUsize,
    e: AtomicUsize,
    f: AtomicUsize,
    g: AtomicUsize,
    h: AtomicUsize,
}
impl Atom for Normal {
    fn new() -> Self {
        Normal {
            a: AtomicUsize::new(0),
            b: AtomicUsize::new(0),
            c: AtomicUsize::new(0),
            d: AtomicUsize::new(0),
            e: AtomicUsize::new(0),
            f: AtomicUsize::new(0),
            g: AtomicUsize::new(0),
            h: AtomicUsize::new(0),
        }
    }
    fn get_ref(&self, ith: usize) -> &AtomicUsize {
        match ith {
            0 => &self.a,
            1 => &self.b,
            2 => &self.c,
            3 => &self.d,
            4 => &self.e,
            5 => &self.f,
            6 => &self.g,
            7 => &self.h,
            _ => unreachable!(),
        }
    }
}

fn main() {
    println!("AtomicUsize: {}!", size_of::<AtomicUsize>());
    println!("aligned: {}!", size_of::<CacheLineAware>());
    println!("norma-sized: {}!", size_of::<NormalSized>());
    println!("normal: {}!", size_of::<Normal>());
}

#[bench]
fn normal1(b: &mut Bencher) {
    work::<Normal>(b, 1);
}
#[bench]
fn normal2(b: &mut Bencher) {
    work::<Normal>(b, 2);
}
#[bench]
fn normal3(b: &mut Bencher) {
    work::<Normal>(b, 3);
}
#[bench]
fn normal4(b: &mut Bencher) {
    work::<Normal>(b, 4);
}
#[bench]
fn normal5(b: &mut Bencher) {
    work::<Normal>(b, 5);
}
#[bench]
fn normal6(b: &mut Bencher) {
    work::<Normal>(b, 6);
}
#[bench]
fn normal7(b: &mut Bencher) {
    work::<Normal>(b, 7);
}
#[bench]
fn normal8(b: &mut Bencher) {
    work::<Normal>(b, 8);
}

#[bench]
fn normal_sized1(b: &mut Bencher) {
    work::<NormalSized>(b, 1);
}
#[bench]
fn normal_sized2(b: &mut Bencher) {
    work::<NormalSized>(b, 2);
}
#[bench]
fn normal_sized3(b: &mut Bencher) {
    work::<NormalSized>(b, 3);
}
#[bench]
fn normal_sized4(b: &mut Bencher) {
    work::<NormalSized>(b, 4);
}
#[bench]
fn normal_sized5(b: &mut Bencher) {
    work::<NormalSized>(b, 5);
}
#[bench]
fn normal_sized6(b: &mut Bencher) {
    work::<NormalSized>(b, 6);
}
#[bench]
fn normal_sized7(b: &mut Bencher) {
    work::<NormalSized>(b, 7);
}
#[bench]
fn normal_sized8(b: &mut Bencher) {
    work::<NormalSized>(b, 8);
}

#[bench]
fn cache_line_aware1(b: &mut Bencher) {
    work::<CacheLineAware>(b, 1);
}
#[bench]
fn cache_line_aware2(b: &mut Bencher) {
    work::<CacheLineAware>(b, 2);
}
#[bench]
fn cache_line_aware3(b: &mut Bencher) {
    work::<CacheLineAware>(b, 3);
}
#[bench]
fn cache_line_aware4(b: &mut Bencher) {
    work::<CacheLineAware>(b, 4);
}
#[bench]
fn cache_line_aware5(b: &mut Bencher) {
    work::<CacheLineAware>(b, 5);
}
#[bench]
fn cache_line_aware6(b: &mut Bencher) {
    work::<CacheLineAware>(b, 6);
}
#[bench]
fn cache_line_aware7(b: &mut Bencher) {
    work::<CacheLineAware>(b, 7);
}
#[bench]
fn cache_line_aware8(b: &mut Bencher) {
    work::<CacheLineAware>(b, 8);
}

fn work<T: Atom>(b: &mut Bencher, n_threads: usize) {
    let pool = threadpool::Builder::new().num_threads(n_threads).build();
    b.iter(|| {
        let atom: T = Atom::new();
        let mut ax = vec![];
        for i in 0..n_threads {
            let r = unsafe { transmute::<&AtomicUsize, &'static AtomicUsize>(atom.get_ref(i)) };
            ax.push(r);
            pool.execute(move || {
                for i in 0..1_000_000 {
                    black_box(r.store(i, Ordering::Relaxed));
                    //black_box(r.store(i, Ordering::SeqCst));
                }
            });
        }

        pool.join();
    });
}
