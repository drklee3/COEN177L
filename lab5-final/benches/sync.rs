#![feature(test, integer_atomics)]

extern crate test;
extern crate threadpool;
extern crate rand;

use test::Bencher;
use std::cell::Cell;
use std::rc::Rc;

use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{Ordering, AtomicPtr, AtomicU64};

use rand::prelude::*;

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

fn bench_mutex_threads(threads: usize, units: u64, locked_percentage: f64) {
  let units_thread = units / threads as u64;            // units per thread
  let units_locked = (units as f64 * locked_percentage) as u64;         // units during mutex locked
  let units_unlocked = (units as f64 * (1.0 - locked_percentage)) as u64; // units during mutex unlocked
  let data = Arc::new(Mutex::new(0u64));
  // create a new threadpool with given thread size
  let pool = threadpool::Builder::new()
    .num_threads(threads)
    .build();

  println!("{}", pool.max_count());
  
  // run on the threadpool
  pool.execute(move || {
    let data = data.clone();
    let mut rng = thread_rng();

    let mut units_left = units_thread;
    while units_left > 0 {
      // run units unlocked
      let rand_units_unlocked = rng.gen_range(0, units_unlocked);
      for _ in 0..rand_units_unlocked {
        let _ = rng.gen::<u32>();
      }

      // update units left, use saturating_sub to prevent underflow
      units_left = units_left.saturating_sub(rand_units_unlocked);

      // run units while locked
      let mut lock = data.lock().expect("Failed to get lock");

      let rand_units_locked = rng.gen_range(0, units_locked);
      for _ in 0..rand_units_locked {
        let _ = rng.gen::<u32>();
      }

      // update units left
      units_left = units_left.saturating_sub(rand_units_locked);

      // update data in mutex
      *lock += rand_units_unlocked + rand_units_locked;
    }
  });

  pool.join();
}

macro_rules! benchtest {
  ($name: ident, $threads: expr, $units: expr, $locked_percentage: expr) => (
    #[bench]
    fn $name(b: &mut Bencher) {
        b.iter(|| bench_mutex_threads($threads, $units, $locked_percentage))
    }
  )
}

benchtest!{mutex_1t_25d, 1, 1000, 0.25}
benchtest!{mutex_2t_25d, 2, 1000, 0.25}
benchtest!{mutex_3t_25d, 3, 1000, 0.25}
benchtest!{mutex_4t_25d, 4, 1000, 0.25}
benchtest!{mutex_5t_25d, 5, 1000, 0.25}
benchtest!{mutex_6t_25d, 6, 1000, 0.25}
benchtest!{mutex_7t_25d, 7, 1000, 0.25}
benchtest!{mutex_8t_25d, 8, 1000, 0.25}

benchtest!{mutex_1t_50d, 1, 1000, 0.50}
benchtest!{mutex_2t_50d, 2, 1000, 0.50}
benchtest!{mutex_3t_50d, 3, 1000, 0.50}
benchtest!{mutex_4t_50d, 4, 1000, 0.50}
benchtest!{mutex_5t_50d, 5, 1000, 0.50}
benchtest!{mutex_6t_50d, 6, 1000, 0.50}
benchtest!{mutex_7t_50d, 7, 1000, 0.50}
benchtest!{mutex_8t_50d, 8, 1000, 0.50}

benchtest!{mutex_1t_75d, 1, 1000, 0.75}
benchtest!{mutex_2t_75d, 2, 1000, 0.75}
benchtest!{mutex_3t_75d, 3, 1000, 0.75}
benchtest!{mutex_4t_75d, 4, 1000, 0.75}
benchtest!{mutex_5t_75d, 5, 1000, 0.75}
benchtest!{mutex_6t_75d, 6, 1000, 0.75}
benchtest!{mutex_7t_75d, 7, 1000, 0.75}
benchtest!{mutex_8t_75d, 8, 1000, 0.75}

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
