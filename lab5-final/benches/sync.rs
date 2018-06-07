#![feature(test, integer_atomics)]

extern crate test;

use test::Bencher;
use std::cell::Cell;
use std::rc::Rc;

use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{Ordering, AtomicPtr, AtomicU64};

const NUM_ITERS: u64 = 1_000_000;

#[bench]
fn bench_no_sync(b: &mut Bencher) {
  b.iter(|| {
    let cell = Cell::new(0u64);
    for _ in 0..NUM_ITERS {
      let x = cell.get();
      test::black_box(cell.set(x + 1));
    }
  });
}

#[bench]
fn bench_rc(b: &mut Bencher) {
  b.iter(|| {
    let cell = Rc::new(Cell::new(0u64));
    let another = cell.clone();
    for _ in 0..NUM_ITERS {
      let x = another.get();
      test::black_box(another.set(x + 1));
    }
  });
}

#[bench]
fn bench_arc(b: &mut Bencher) {
  b.iter(|| {
    let cell = Arc::new(Cell::new(0u64));
    let another = cell.clone();
    for _ in 0..NUM_ITERS {
      let x = another.get();
      test::black_box(another.set(x + 1));
    }
  });
}

#[bench]
fn bench_mutex(b: &mut Bencher) {
  b.iter(|| {
    let data = Mutex::new(0u64);
    for _ in 0..NUM_ITERS {
      let mut x = data.lock().unwrap();
      *x += 1;
    }
  })
}

#[bench]
fn bench_rwlock_write(b: &mut Bencher) {
  b.iter(|| {
    let data = RwLock::new(0u64);
    for _ in 0..NUM_ITERS {
      let mut writer = data.write().unwrap();
      *writer += 1;
    }
  })
}

#[bench]
fn bench_rwlock_read(b: &mut Bencher) {
  b.iter(|| {
    let data = RwLock::new(0u64);
    for _ in 0..NUM_ITERS {
      let reader = data.read().unwrap();
      test::black_box(*reader + 1);
    }
  })
}
