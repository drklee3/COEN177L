#![feature(test, integer_atomics, concat_idents)]

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
const NUM_UNITS: u64 = 1_000;

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

  // spawn number of threads
  for _ in 0..threads {
    let data = data.clone();
    
    // run on the threadpool
    pool.execute(move || {
      let mut rng = thread_rng();
      let mut units_left = units_thread;
      while units_left > 0 {
        // calculate units to run while unlocked
        let rand_units_unlocked = if units_unlocked > 0 {
          rng.gen_range(0, units_unlocked)
        } else {
          0
        };

        // run units unlocked
        for _ in 0..rand_units_unlocked {
          let _ = rng.gen::<u32>();
        }

        // update units left, use saturating_sub to prevent underflow
        units_left = units_left.saturating_sub(rand_units_unlocked);

        // lock data
        let mut lock = data.lock().expect("Failed to get lock");

        // calculate units to run while locked
        let rand_units_locked = if units_locked > 0 {
          rng.gen_range(0, units_locked)
        } else {
          0
        };

        // run units while locked
        for _ in 0..rand_units_locked {
          let _ = rng.gen::<u32>();
        }

        // update units left
        units_left = units_left.saturating_sub(rand_units_locked);

        // update data in mutex
        *lock += rand_units_unlocked + rand_units_locked;
      }
    });
  }

  // block until all threads complete
  pool.join();
}

macro_rules! benchtest {
  ($name: ident, $threads: expr, $locked_percentage: expr) => (
    #[bench]
    fn $name(b: &mut Bencher) {
        b.iter(|| bench_mutex_threads($threads, NUM_UNITS, $locked_percentage))
    }
  )
}

benchtest!{mutex_1t_000p, 1, 0.00}
benchtest!{mutex_1t_001p, 1, 0.01}
benchtest!{mutex_1t_002p, 1, 0.02}
benchtest!{mutex_1t_003p, 1, 0.03}
benchtest!{mutex_1t_004p, 1, 0.04}
benchtest!{mutex_1t_005p, 1, 0.05}
benchtest!{mutex_1t_006p, 1, 0.06}
benchtest!{mutex_1t_007p, 1, 0.07}
benchtest!{mutex_1t_008p, 1, 0.08}
benchtest!{mutex_1t_009p, 1, 0.09}
benchtest!{mutex_1t_010p, 1, 0.10}
benchtest!{mutex_1t_011p, 1, 0.11}
benchtest!{mutex_1t_012p, 1, 0.12}
benchtest!{mutex_1t_013p, 1, 0.13}
benchtest!{mutex_1t_014p, 1, 0.14}
benchtest!{mutex_1t_015p, 1, 0.15}
benchtest!{mutex_1t_016p, 1, 0.16}
benchtest!{mutex_1t_017p, 1, 0.17}
benchtest!{mutex_1t_018p, 1, 0.18}
benchtest!{mutex_1t_019p, 1, 0.19}
benchtest!{mutex_1t_020p, 1, 0.20}
benchtest!{mutex_1t_021p, 1, 0.21}
benchtest!{mutex_1t_022p, 1, 0.22}
benchtest!{mutex_1t_023p, 1, 0.23}
benchtest!{mutex_1t_024p, 1, 0.24}
benchtest!{mutex_1t_025p, 1, 0.25}
benchtest!{mutex_1t_026p, 1, 0.26}
benchtest!{mutex_1t_027p, 1, 0.27}
benchtest!{mutex_1t_028p, 1, 0.28}
benchtest!{mutex_1t_029p, 1, 0.29}
benchtest!{mutex_1t_030p, 1, 0.30}
benchtest!{mutex_1t_031p, 1, 0.31}
benchtest!{mutex_1t_032p, 1, 0.32}
benchtest!{mutex_1t_033p, 1, 0.33}
benchtest!{mutex_1t_034p, 1, 0.34}
benchtest!{mutex_1t_035p, 1, 0.35}
benchtest!{mutex_1t_036p, 1, 0.36}
benchtest!{mutex_1t_037p, 1, 0.37}
benchtest!{mutex_1t_038p, 1, 0.38}
benchtest!{mutex_1t_039p, 1, 0.39}
benchtest!{mutex_1t_040p, 1, 0.40}
benchtest!{mutex_1t_041p, 1, 0.41}
benchtest!{mutex_1t_042p, 1, 0.42}
benchtest!{mutex_1t_043p, 1, 0.43}
benchtest!{mutex_1t_044p, 1, 0.44}
benchtest!{mutex_1t_045p, 1, 0.45}
benchtest!{mutex_1t_046p, 1, 0.46}
benchtest!{mutex_1t_047p, 1, 0.47}
benchtest!{mutex_1t_048p, 1, 0.48}
benchtest!{mutex_1t_049p, 1, 0.49}
benchtest!{mutex_1t_050p, 1, 0.50}
benchtest!{mutex_1t_051p, 1, 0.51}
benchtest!{mutex_1t_052p, 1, 0.52}
benchtest!{mutex_1t_053p, 1, 0.53}
benchtest!{mutex_1t_054p, 1, 0.54}
benchtest!{mutex_1t_055p, 1, 0.55}
benchtest!{mutex_1t_056p, 1, 0.56}
benchtest!{mutex_1t_057p, 1, 0.57}
benchtest!{mutex_1t_058p, 1, 0.58}
benchtest!{mutex_1t_059p, 1, 0.59}
benchtest!{mutex_1t_060p, 1, 0.60}
benchtest!{mutex_1t_061p, 1, 0.61}
benchtest!{mutex_1t_062p, 1, 0.62}
benchtest!{mutex_1t_063p, 1, 0.63}
benchtest!{mutex_1t_064p, 1, 0.64}
benchtest!{mutex_1t_065p, 1, 0.65}
benchtest!{mutex_1t_066p, 1, 0.66}
benchtest!{mutex_1t_067p, 1, 0.67}
benchtest!{mutex_1t_068p, 1, 0.68}
benchtest!{mutex_1t_069p, 1, 0.69}
benchtest!{mutex_1t_070p, 1, 0.70}
benchtest!{mutex_1t_071p, 1, 0.71}
benchtest!{mutex_1t_072p, 1, 0.72}
benchtest!{mutex_1t_073p, 1, 0.73}
benchtest!{mutex_1t_074p, 1, 0.74}
benchtest!{mutex_1t_075p, 1, 0.75}
benchtest!{mutex_1t_076p, 1, 0.76}
benchtest!{mutex_1t_077p, 1, 0.77}
benchtest!{mutex_1t_078p, 1, 0.78}
benchtest!{mutex_1t_079p, 1, 0.79}
benchtest!{mutex_1t_080p, 1, 0.80}
benchtest!{mutex_1t_081p, 1, 0.81}
benchtest!{mutex_1t_082p, 1, 0.82}
benchtest!{mutex_1t_083p, 1, 0.83}
benchtest!{mutex_1t_084p, 1, 0.84}
benchtest!{mutex_1t_085p, 1, 0.85}
benchtest!{mutex_1t_086p, 1, 0.86}
benchtest!{mutex_1t_087p, 1, 0.87}
benchtest!{mutex_1t_088p, 1, 0.88}
benchtest!{mutex_1t_089p, 1, 0.89}
benchtest!{mutex_1t_090p, 1, 0.90}
benchtest!{mutex_1t_091p, 1, 0.91}
benchtest!{mutex_1t_092p, 1, 0.92}
benchtest!{mutex_1t_093p, 1, 0.93}
benchtest!{mutex_1t_094p, 1, 0.94}
benchtest!{mutex_1t_095p, 1, 0.95}
benchtest!{mutex_1t_096p, 1, 0.96}
benchtest!{mutex_1t_097p, 1, 0.97}
benchtest!{mutex_1t_098p, 1, 0.98}
benchtest!{mutex_1t_099p, 1, 0.99}
benchtest!{mutex_1t_100p, 1, 1.00}

benchtest!{mutex_2t_000p, 2, 0.00}
benchtest!{mutex_2t_001p, 2, 0.01}
benchtest!{mutex_2t_002p, 2, 0.02}
benchtest!{mutex_2t_003p, 2, 0.03}
benchtest!{mutex_2t_004p, 2, 0.04}
benchtest!{mutex_2t_005p, 2, 0.05}
benchtest!{mutex_2t_006p, 2, 0.06}
benchtest!{mutex_2t_007p, 2, 0.07}
benchtest!{mutex_2t_008p, 2, 0.08}
benchtest!{mutex_2t_009p, 2, 0.09}
benchtest!{mutex_2t_010p, 2, 0.10}
benchtest!{mutex_2t_011p, 2, 0.11}
benchtest!{mutex_2t_012p, 2, 0.12}
benchtest!{mutex_2t_013p, 2, 0.13}
benchtest!{mutex_2t_014p, 2, 0.14}
benchtest!{mutex_2t_015p, 2, 0.15}
benchtest!{mutex_2t_016p, 2, 0.16}
benchtest!{mutex_2t_017p, 2, 0.17}
benchtest!{mutex_2t_018p, 2, 0.18}
benchtest!{mutex_2t_019p, 2, 0.19}
benchtest!{mutex_2t_020p, 2, 0.20}
benchtest!{mutex_2t_021p, 2, 0.21}
benchtest!{mutex_2t_022p, 2, 0.22}
benchtest!{mutex_2t_023p, 2, 0.23}
benchtest!{mutex_2t_024p, 2, 0.24}
benchtest!{mutex_2t_025p, 2, 0.25}
benchtest!{mutex_2t_026p, 2, 0.26}
benchtest!{mutex_2t_027p, 2, 0.27}
benchtest!{mutex_2t_028p, 2, 0.28}
benchtest!{mutex_2t_029p, 2, 0.29}
benchtest!{mutex_2t_030p, 2, 0.30}
benchtest!{mutex_2t_031p, 2, 0.31}
benchtest!{mutex_2t_032p, 2, 0.32}
benchtest!{mutex_2t_033p, 2, 0.33}
benchtest!{mutex_2t_034p, 2, 0.34}
benchtest!{mutex_2t_035p, 2, 0.35}
benchtest!{mutex_2t_036p, 2, 0.36}
benchtest!{mutex_2t_037p, 2, 0.37}
benchtest!{mutex_2t_038p, 2, 0.38}
benchtest!{mutex_2t_039p, 2, 0.39}
benchtest!{mutex_2t_040p, 2, 0.40}
benchtest!{mutex_2t_041p, 2, 0.41}
benchtest!{mutex_2t_042p, 2, 0.42}
benchtest!{mutex_2t_043p, 2, 0.43}
benchtest!{mutex_2t_044p, 2, 0.44}
benchtest!{mutex_2t_045p, 2, 0.45}
benchtest!{mutex_2t_046p, 2, 0.46}
benchtest!{mutex_2t_047p, 2, 0.47}
benchtest!{mutex_2t_048p, 2, 0.48}
benchtest!{mutex_2t_049p, 2, 0.49}
benchtest!{mutex_2t_050p, 2, 0.50}
benchtest!{mutex_2t_051p, 2, 0.51}
benchtest!{mutex_2t_052p, 2, 0.52}
benchtest!{mutex_2t_053p, 2, 0.53}
benchtest!{mutex_2t_054p, 2, 0.54}
benchtest!{mutex_2t_055p, 2, 0.55}
benchtest!{mutex_2t_056p, 2, 0.56}
benchtest!{mutex_2t_057p, 2, 0.57}
benchtest!{mutex_2t_058p, 2, 0.58}
benchtest!{mutex_2t_059p, 2, 0.59}
benchtest!{mutex_2t_060p, 2, 0.60}
benchtest!{mutex_2t_061p, 2, 0.61}
benchtest!{mutex_2t_062p, 2, 0.62}
benchtest!{mutex_2t_063p, 2, 0.63}
benchtest!{mutex_2t_064p, 2, 0.64}
benchtest!{mutex_2t_065p, 2, 0.65}
benchtest!{mutex_2t_066p, 2, 0.66}
benchtest!{mutex_2t_067p, 2, 0.67}
benchtest!{mutex_2t_068p, 2, 0.68}
benchtest!{mutex_2t_069p, 2, 0.69}
benchtest!{mutex_2t_070p, 2, 0.70}
benchtest!{mutex_2t_071p, 2, 0.71}
benchtest!{mutex_2t_072p, 2, 0.72}
benchtest!{mutex_2t_073p, 2, 0.73}
benchtest!{mutex_2t_074p, 2, 0.74}
benchtest!{mutex_2t_075p, 2, 0.75}
benchtest!{mutex_2t_076p, 2, 0.76}
benchtest!{mutex_2t_077p, 2, 0.77}
benchtest!{mutex_2t_078p, 2, 0.78}
benchtest!{mutex_2t_079p, 2, 0.79}
benchtest!{mutex_2t_080p, 2, 0.80}
benchtest!{mutex_2t_081p, 2, 0.81}
benchtest!{mutex_2t_082p, 2, 0.82}
benchtest!{mutex_2t_083p, 2, 0.83}
benchtest!{mutex_2t_084p, 2, 0.84}
benchtest!{mutex_2t_085p, 2, 0.85}
benchtest!{mutex_2t_086p, 2, 0.86}
benchtest!{mutex_2t_087p, 2, 0.87}
benchtest!{mutex_2t_088p, 2, 0.88}
benchtest!{mutex_2t_089p, 2, 0.89}
benchtest!{mutex_2t_090p, 2, 0.90}
benchtest!{mutex_2t_091p, 2, 0.91}
benchtest!{mutex_2t_092p, 2, 0.92}
benchtest!{mutex_2t_093p, 2, 0.93}
benchtest!{mutex_2t_094p, 2, 0.94}
benchtest!{mutex_2t_095p, 2, 0.95}
benchtest!{mutex_2t_096p, 2, 0.96}
benchtest!{mutex_2t_097p, 2, 0.97}
benchtest!{mutex_2t_098p, 2, 0.98}
benchtest!{mutex_2t_099p, 2, 0.99}
benchtest!{mutex_2t_100p, 2, 1.00}

benchtest!{mutex_3t_000p, 3, 0.00}
benchtest!{mutex_3t_001p, 3, 0.01}
benchtest!{mutex_3t_002p, 3, 0.02}
benchtest!{mutex_3t_003p, 3, 0.03}
benchtest!{mutex_3t_004p, 3, 0.04}
benchtest!{mutex_3t_005p, 3, 0.05}
benchtest!{mutex_3t_006p, 3, 0.06}
benchtest!{mutex_3t_007p, 3, 0.07}
benchtest!{mutex_3t_008p, 3, 0.08}
benchtest!{mutex_3t_009p, 3, 0.09}
benchtest!{mutex_3t_010p, 3, 0.10}
benchtest!{mutex_3t_011p, 3, 0.11}
benchtest!{mutex_3t_012p, 3, 0.12}
benchtest!{mutex_3t_013p, 3, 0.13}
benchtest!{mutex_3t_014p, 3, 0.14}
benchtest!{mutex_3t_015p, 3, 0.15}
benchtest!{mutex_3t_016p, 3, 0.16}
benchtest!{mutex_3t_017p, 3, 0.17}
benchtest!{mutex_3t_018p, 3, 0.18}
benchtest!{mutex_3t_019p, 3, 0.19}
benchtest!{mutex_3t_020p, 3, 0.20}
benchtest!{mutex_3t_021p, 3, 0.21}
benchtest!{mutex_3t_022p, 3, 0.22}
benchtest!{mutex_3t_023p, 3, 0.23}
benchtest!{mutex_3t_024p, 3, 0.24}
benchtest!{mutex_3t_025p, 3, 0.25}
benchtest!{mutex_3t_026p, 3, 0.26}
benchtest!{mutex_3t_027p, 3, 0.27}
benchtest!{mutex_3t_028p, 3, 0.28}
benchtest!{mutex_3t_029p, 3, 0.29}
benchtest!{mutex_3t_030p, 3, 0.30}
benchtest!{mutex_3t_031p, 3, 0.31}
benchtest!{mutex_3t_032p, 3, 0.32}
benchtest!{mutex_3t_033p, 3, 0.33}
benchtest!{mutex_3t_034p, 3, 0.34}
benchtest!{mutex_3t_035p, 3, 0.35}
benchtest!{mutex_3t_036p, 3, 0.36}
benchtest!{mutex_3t_037p, 3, 0.37}
benchtest!{mutex_3t_038p, 3, 0.38}
benchtest!{mutex_3t_039p, 3, 0.39}
benchtest!{mutex_3t_040p, 3, 0.40}
benchtest!{mutex_3t_041p, 3, 0.41}
benchtest!{mutex_3t_042p, 3, 0.42}
benchtest!{mutex_3t_043p, 3, 0.43}
benchtest!{mutex_3t_044p, 3, 0.44}
benchtest!{mutex_3t_045p, 3, 0.45}
benchtest!{mutex_3t_046p, 3, 0.46}
benchtest!{mutex_3t_047p, 3, 0.47}
benchtest!{mutex_3t_048p, 3, 0.48}
benchtest!{mutex_3t_049p, 3, 0.49}
benchtest!{mutex_3t_050p, 3, 0.50}
benchtest!{mutex_3t_051p, 3, 0.51}
benchtest!{mutex_3t_052p, 3, 0.52}
benchtest!{mutex_3t_053p, 3, 0.53}
benchtest!{mutex_3t_054p, 3, 0.54}
benchtest!{mutex_3t_055p, 3, 0.55}
benchtest!{mutex_3t_056p, 3, 0.56}
benchtest!{mutex_3t_057p, 3, 0.57}
benchtest!{mutex_3t_058p, 3, 0.58}
benchtest!{mutex_3t_059p, 3, 0.59}
benchtest!{mutex_3t_060p, 3, 0.60}
benchtest!{mutex_3t_061p, 3, 0.61}
benchtest!{mutex_3t_062p, 3, 0.62}
benchtest!{mutex_3t_063p, 3, 0.63}
benchtest!{mutex_3t_064p, 3, 0.64}
benchtest!{mutex_3t_065p, 3, 0.65}
benchtest!{mutex_3t_066p, 3, 0.66}
benchtest!{mutex_3t_067p, 3, 0.67}
benchtest!{mutex_3t_068p, 3, 0.68}
benchtest!{mutex_3t_069p, 3, 0.69}
benchtest!{mutex_3t_070p, 3, 0.70}
benchtest!{mutex_3t_071p, 3, 0.71}
benchtest!{mutex_3t_072p, 3, 0.72}
benchtest!{mutex_3t_073p, 3, 0.73}
benchtest!{mutex_3t_074p, 3, 0.74}
benchtest!{mutex_3t_075p, 3, 0.75}
benchtest!{mutex_3t_076p, 3, 0.76}
benchtest!{mutex_3t_077p, 3, 0.77}
benchtest!{mutex_3t_078p, 3, 0.78}
benchtest!{mutex_3t_079p, 3, 0.79}
benchtest!{mutex_3t_080p, 3, 0.80}
benchtest!{mutex_3t_081p, 3, 0.81}
benchtest!{mutex_3t_082p, 3, 0.82}
benchtest!{mutex_3t_083p, 3, 0.83}
benchtest!{mutex_3t_084p, 3, 0.84}
benchtest!{mutex_3t_085p, 3, 0.85}
benchtest!{mutex_3t_086p, 3, 0.86}
benchtest!{mutex_3t_087p, 3, 0.87}
benchtest!{mutex_3t_088p, 3, 0.88}
benchtest!{mutex_3t_089p, 3, 0.89}
benchtest!{mutex_3t_090p, 3, 0.90}
benchtest!{mutex_3t_091p, 3, 0.91}
benchtest!{mutex_3t_092p, 3, 0.92}
benchtest!{mutex_3t_093p, 3, 0.93}
benchtest!{mutex_3t_094p, 3, 0.94}
benchtest!{mutex_3t_095p, 3, 0.95}
benchtest!{mutex_3t_096p, 3, 0.96}
benchtest!{mutex_3t_097p, 3, 0.97}
benchtest!{mutex_3t_098p, 3, 0.98}
benchtest!{mutex_3t_099p, 3, 0.99}
benchtest!{mutex_3t_100p, 3, 1.00}

benchtest!{mutex_4t_000p, 4, 0.00}
benchtest!{mutex_4t_001p, 4, 0.01}
benchtest!{mutex_4t_002p, 4, 0.02}
benchtest!{mutex_4t_003p, 4, 0.03}
benchtest!{mutex_4t_004p, 4, 0.04}
benchtest!{mutex_4t_005p, 4, 0.05}
benchtest!{mutex_4t_006p, 4, 0.06}
benchtest!{mutex_4t_007p, 4, 0.07}
benchtest!{mutex_4t_008p, 4, 0.08}
benchtest!{mutex_4t_009p, 4, 0.09}
benchtest!{mutex_4t_010p, 4, 0.10}
benchtest!{mutex_4t_011p, 4, 0.11}
benchtest!{mutex_4t_012p, 4, 0.12}
benchtest!{mutex_4t_013p, 4, 0.13}
benchtest!{mutex_4t_014p, 4, 0.14}
benchtest!{mutex_4t_015p, 4, 0.15}
benchtest!{mutex_4t_016p, 4, 0.16}
benchtest!{mutex_4t_017p, 4, 0.17}
benchtest!{mutex_4t_018p, 4, 0.18}
benchtest!{mutex_4t_019p, 4, 0.19}
benchtest!{mutex_4t_020p, 4, 0.20}
benchtest!{mutex_4t_021p, 4, 0.21}
benchtest!{mutex_4t_022p, 4, 0.22}
benchtest!{mutex_4t_023p, 4, 0.23}
benchtest!{mutex_4t_024p, 4, 0.24}
benchtest!{mutex_4t_025p, 4, 0.25}
benchtest!{mutex_4t_026p, 4, 0.26}
benchtest!{mutex_4t_027p, 4, 0.27}
benchtest!{mutex_4t_028p, 4, 0.28}
benchtest!{mutex_4t_029p, 4, 0.29}
benchtest!{mutex_4t_030p, 4, 0.30}
benchtest!{mutex_4t_031p, 4, 0.31}
benchtest!{mutex_4t_032p, 4, 0.32}
benchtest!{mutex_4t_033p, 4, 0.33}
benchtest!{mutex_4t_034p, 4, 0.34}
benchtest!{mutex_4t_035p, 4, 0.35}
benchtest!{mutex_4t_036p, 4, 0.36}
benchtest!{mutex_4t_037p, 4, 0.37}
benchtest!{mutex_4t_038p, 4, 0.38}
benchtest!{mutex_4t_039p, 4, 0.39}
benchtest!{mutex_4t_040p, 4, 0.40}
benchtest!{mutex_4t_041p, 4, 0.41}
benchtest!{mutex_4t_042p, 4, 0.42}
benchtest!{mutex_4t_043p, 4, 0.43}
benchtest!{mutex_4t_044p, 4, 0.44}
benchtest!{mutex_4t_045p, 4, 0.45}
benchtest!{mutex_4t_046p, 4, 0.46}
benchtest!{mutex_4t_047p, 4, 0.47}
benchtest!{mutex_4t_048p, 4, 0.48}
benchtest!{mutex_4t_049p, 4, 0.49}
benchtest!{mutex_4t_050p, 4, 0.50}
benchtest!{mutex_4t_051p, 4, 0.51}
benchtest!{mutex_4t_052p, 4, 0.52}
benchtest!{mutex_4t_053p, 4, 0.53}
benchtest!{mutex_4t_054p, 4, 0.54}
benchtest!{mutex_4t_055p, 4, 0.55}
benchtest!{mutex_4t_056p, 4, 0.56}
benchtest!{mutex_4t_057p, 4, 0.57}
benchtest!{mutex_4t_058p, 4, 0.58}
benchtest!{mutex_4t_059p, 4, 0.59}
benchtest!{mutex_4t_060p, 4, 0.60}
benchtest!{mutex_4t_061p, 4, 0.61}
benchtest!{mutex_4t_062p, 4, 0.62}
benchtest!{mutex_4t_063p, 4, 0.63}
benchtest!{mutex_4t_064p, 4, 0.64}
benchtest!{mutex_4t_065p, 4, 0.65}
benchtest!{mutex_4t_066p, 4, 0.66}
benchtest!{mutex_4t_067p, 4, 0.67}
benchtest!{mutex_4t_068p, 4, 0.68}
benchtest!{mutex_4t_069p, 4, 0.69}
benchtest!{mutex_4t_070p, 4, 0.70}
benchtest!{mutex_4t_071p, 4, 0.71}
benchtest!{mutex_4t_072p, 4, 0.72}
benchtest!{mutex_4t_073p, 4, 0.73}
benchtest!{mutex_4t_074p, 4, 0.74}
benchtest!{mutex_4t_075p, 4, 0.75}
benchtest!{mutex_4t_076p, 4, 0.76}
benchtest!{mutex_4t_077p, 4, 0.77}
benchtest!{mutex_4t_078p, 4, 0.78}
benchtest!{mutex_4t_079p, 4, 0.79}
benchtest!{mutex_4t_080p, 4, 0.80}
benchtest!{mutex_4t_081p, 4, 0.81}
benchtest!{mutex_4t_082p, 4, 0.82}
benchtest!{mutex_4t_083p, 4, 0.83}
benchtest!{mutex_4t_084p, 4, 0.84}
benchtest!{mutex_4t_085p, 4, 0.85}
benchtest!{mutex_4t_086p, 4, 0.86}
benchtest!{mutex_4t_087p, 4, 0.87}
benchtest!{mutex_4t_088p, 4, 0.88}
benchtest!{mutex_4t_089p, 4, 0.89}
benchtest!{mutex_4t_090p, 4, 0.90}
benchtest!{mutex_4t_091p, 4, 0.91}
benchtest!{mutex_4t_092p, 4, 0.92}
benchtest!{mutex_4t_093p, 4, 0.93}
benchtest!{mutex_4t_094p, 4, 0.94}
benchtest!{mutex_4t_095p, 4, 0.95}
benchtest!{mutex_4t_096p, 4, 0.96}
benchtest!{mutex_4t_097p, 4, 0.97}
benchtest!{mutex_4t_098p, 4, 0.98}
benchtest!{mutex_4t_099p, 4, 0.99}
benchtest!{mutex_4t_100p, 4, 1.00}

benchtest!{mutex_5t_000p, 5, 0.00}
benchtest!{mutex_5t_001p, 5, 0.01}
benchtest!{mutex_5t_002p, 5, 0.02}
benchtest!{mutex_5t_003p, 5, 0.03}
benchtest!{mutex_5t_004p, 5, 0.04}
benchtest!{mutex_5t_005p, 5, 0.05}
benchtest!{mutex_5t_006p, 5, 0.06}
benchtest!{mutex_5t_007p, 5, 0.07}
benchtest!{mutex_5t_008p, 5, 0.08}
benchtest!{mutex_5t_009p, 5, 0.09}
benchtest!{mutex_5t_010p, 5, 0.10}
benchtest!{mutex_5t_011p, 5, 0.11}
benchtest!{mutex_5t_012p, 5, 0.12}
benchtest!{mutex_5t_013p, 5, 0.13}
benchtest!{mutex_5t_014p, 5, 0.14}
benchtest!{mutex_5t_015p, 5, 0.15}
benchtest!{mutex_5t_016p, 5, 0.16}
benchtest!{mutex_5t_017p, 5, 0.17}
benchtest!{mutex_5t_018p, 5, 0.18}
benchtest!{mutex_5t_019p, 5, 0.19}
benchtest!{mutex_5t_020p, 5, 0.20}
benchtest!{mutex_5t_021p, 5, 0.21}
benchtest!{mutex_5t_022p, 5, 0.22}
benchtest!{mutex_5t_023p, 5, 0.23}
benchtest!{mutex_5t_024p, 5, 0.24}
benchtest!{mutex_5t_025p, 5, 0.25}
benchtest!{mutex_5t_026p, 5, 0.26}
benchtest!{mutex_5t_027p, 5, 0.27}
benchtest!{mutex_5t_028p, 5, 0.28}
benchtest!{mutex_5t_029p, 5, 0.29}
benchtest!{mutex_5t_030p, 5, 0.30}
benchtest!{mutex_5t_031p, 5, 0.31}
benchtest!{mutex_5t_032p, 5, 0.32}
benchtest!{mutex_5t_033p, 5, 0.33}
benchtest!{mutex_5t_034p, 5, 0.34}
benchtest!{mutex_5t_035p, 5, 0.35}
benchtest!{mutex_5t_036p, 5, 0.36}
benchtest!{mutex_5t_037p, 5, 0.37}
benchtest!{mutex_5t_038p, 5, 0.38}
benchtest!{mutex_5t_039p, 5, 0.39}
benchtest!{mutex_5t_040p, 5, 0.40}
benchtest!{mutex_5t_041p, 5, 0.41}
benchtest!{mutex_5t_042p, 5, 0.42}
benchtest!{mutex_5t_043p, 5, 0.43}
benchtest!{mutex_5t_044p, 5, 0.44}
benchtest!{mutex_5t_045p, 5, 0.45}
benchtest!{mutex_5t_046p, 5, 0.46}
benchtest!{mutex_5t_047p, 5, 0.47}
benchtest!{mutex_5t_048p, 5, 0.48}
benchtest!{mutex_5t_049p, 5, 0.49}
benchtest!{mutex_5t_050p, 5, 0.50}
benchtest!{mutex_5t_051p, 5, 0.51}
benchtest!{mutex_5t_052p, 5, 0.52}
benchtest!{mutex_5t_053p, 5, 0.53}
benchtest!{mutex_5t_054p, 5, 0.54}
benchtest!{mutex_5t_055p, 5, 0.55}
benchtest!{mutex_5t_056p, 5, 0.56}
benchtest!{mutex_5t_057p, 5, 0.57}
benchtest!{mutex_5t_058p, 5, 0.58}
benchtest!{mutex_5t_059p, 5, 0.59}
benchtest!{mutex_5t_060p, 5, 0.60}
benchtest!{mutex_5t_061p, 5, 0.61}
benchtest!{mutex_5t_062p, 5, 0.62}
benchtest!{mutex_5t_063p, 5, 0.63}
benchtest!{mutex_5t_064p, 5, 0.64}
benchtest!{mutex_5t_065p, 5, 0.65}
benchtest!{mutex_5t_066p, 5, 0.66}
benchtest!{mutex_5t_067p, 5, 0.67}
benchtest!{mutex_5t_068p, 5, 0.68}
benchtest!{mutex_5t_069p, 5, 0.69}
benchtest!{mutex_5t_070p, 5, 0.70}
benchtest!{mutex_5t_071p, 5, 0.71}
benchtest!{mutex_5t_072p, 5, 0.72}
benchtest!{mutex_5t_073p, 5, 0.73}
benchtest!{mutex_5t_074p, 5, 0.74}
benchtest!{mutex_5t_075p, 5, 0.75}
benchtest!{mutex_5t_076p, 5, 0.76}
benchtest!{mutex_5t_077p, 5, 0.77}
benchtest!{mutex_5t_078p, 5, 0.78}
benchtest!{mutex_5t_079p, 5, 0.79}
benchtest!{mutex_5t_080p, 5, 0.80}
benchtest!{mutex_5t_081p, 5, 0.81}
benchtest!{mutex_5t_082p, 5, 0.82}
benchtest!{mutex_5t_083p, 5, 0.83}
benchtest!{mutex_5t_084p, 5, 0.84}
benchtest!{mutex_5t_085p, 5, 0.85}
benchtest!{mutex_5t_086p, 5, 0.86}
benchtest!{mutex_5t_087p, 5, 0.87}
benchtest!{mutex_5t_088p, 5, 0.88}
benchtest!{mutex_5t_089p, 5, 0.89}
benchtest!{mutex_5t_090p, 5, 0.90}
benchtest!{mutex_5t_091p, 5, 0.91}
benchtest!{mutex_5t_092p, 5, 0.92}
benchtest!{mutex_5t_093p, 5, 0.93}
benchtest!{mutex_5t_094p, 5, 0.94}
benchtest!{mutex_5t_095p, 5, 0.95}
benchtest!{mutex_5t_096p, 5, 0.96}
benchtest!{mutex_5t_097p, 5, 0.97}
benchtest!{mutex_5t_098p, 5, 0.98}
benchtest!{mutex_5t_099p, 5, 0.99}
benchtest!{mutex_5t_100p, 5, 1.00}

benchtest!{mutex_6t_000p, 6, 0.00}
benchtest!{mutex_6t_001p, 6, 0.01}
benchtest!{mutex_6t_002p, 6, 0.02}
benchtest!{mutex_6t_003p, 6, 0.03}
benchtest!{mutex_6t_004p, 6, 0.04}
benchtest!{mutex_6t_005p, 6, 0.05}
benchtest!{mutex_6t_006p, 6, 0.06}
benchtest!{mutex_6t_007p, 6, 0.07}
benchtest!{mutex_6t_008p, 6, 0.08}
benchtest!{mutex_6t_009p, 6, 0.09}
benchtest!{mutex_6t_010p, 6, 0.10}
benchtest!{mutex_6t_011p, 6, 0.11}
benchtest!{mutex_6t_012p, 6, 0.12}
benchtest!{mutex_6t_013p, 6, 0.13}
benchtest!{mutex_6t_014p, 6, 0.14}
benchtest!{mutex_6t_015p, 6, 0.15}
benchtest!{mutex_6t_016p, 6, 0.16}
benchtest!{mutex_6t_017p, 6, 0.17}
benchtest!{mutex_6t_018p, 6, 0.18}
benchtest!{mutex_6t_019p, 6, 0.19}
benchtest!{mutex_6t_020p, 6, 0.20}
benchtest!{mutex_6t_021p, 6, 0.21}
benchtest!{mutex_6t_022p, 6, 0.22}
benchtest!{mutex_6t_023p, 6, 0.23}
benchtest!{mutex_6t_024p, 6, 0.24}
benchtest!{mutex_6t_025p, 6, 0.25}
benchtest!{mutex_6t_026p, 6, 0.26}
benchtest!{mutex_6t_027p, 6, 0.27}
benchtest!{mutex_6t_028p, 6, 0.28}
benchtest!{mutex_6t_029p, 6, 0.29}
benchtest!{mutex_6t_030p, 6, 0.30}
benchtest!{mutex_6t_031p, 6, 0.31}
benchtest!{mutex_6t_032p, 6, 0.32}
benchtest!{mutex_6t_033p, 6, 0.33}
benchtest!{mutex_6t_034p, 6, 0.34}
benchtest!{mutex_6t_035p, 6, 0.35}
benchtest!{mutex_6t_036p, 6, 0.36}
benchtest!{mutex_6t_037p, 6, 0.37}
benchtest!{mutex_6t_038p, 6, 0.38}
benchtest!{mutex_6t_039p, 6, 0.39}
benchtest!{mutex_6t_040p, 6, 0.40}
benchtest!{mutex_6t_041p, 6, 0.41}
benchtest!{mutex_6t_042p, 6, 0.42}
benchtest!{mutex_6t_043p, 6, 0.43}
benchtest!{mutex_6t_044p, 6, 0.44}
benchtest!{mutex_6t_045p, 6, 0.45}
benchtest!{mutex_6t_046p, 6, 0.46}
benchtest!{mutex_6t_047p, 6, 0.47}
benchtest!{mutex_6t_048p, 6, 0.48}
benchtest!{mutex_6t_049p, 6, 0.49}
benchtest!{mutex_6t_050p, 6, 0.50}
benchtest!{mutex_6t_051p, 6, 0.51}
benchtest!{mutex_6t_052p, 6, 0.52}
benchtest!{mutex_6t_053p, 6, 0.53}
benchtest!{mutex_6t_054p, 6, 0.54}
benchtest!{mutex_6t_055p, 6, 0.55}
benchtest!{mutex_6t_056p, 6, 0.56}
benchtest!{mutex_6t_057p, 6, 0.57}
benchtest!{mutex_6t_058p, 6, 0.58}
benchtest!{mutex_6t_059p, 6, 0.59}
benchtest!{mutex_6t_060p, 6, 0.60}
benchtest!{mutex_6t_061p, 6, 0.61}
benchtest!{mutex_6t_062p, 6, 0.62}
benchtest!{mutex_6t_063p, 6, 0.63}
benchtest!{mutex_6t_064p, 6, 0.64}
benchtest!{mutex_6t_065p, 6, 0.65}
benchtest!{mutex_6t_066p, 6, 0.66}
benchtest!{mutex_6t_067p, 6, 0.67}
benchtest!{mutex_6t_068p, 6, 0.68}
benchtest!{mutex_6t_069p, 6, 0.69}
benchtest!{mutex_6t_070p, 6, 0.70}
benchtest!{mutex_6t_071p, 6, 0.71}
benchtest!{mutex_6t_072p, 6, 0.72}
benchtest!{mutex_6t_073p, 6, 0.73}
benchtest!{mutex_6t_074p, 6, 0.74}
benchtest!{mutex_6t_075p, 6, 0.75}
benchtest!{mutex_6t_076p, 6, 0.76}
benchtest!{mutex_6t_077p, 6, 0.77}
benchtest!{mutex_6t_078p, 6, 0.78}
benchtest!{mutex_6t_079p, 6, 0.79}
benchtest!{mutex_6t_080p, 6, 0.80}
benchtest!{mutex_6t_081p, 6, 0.81}
benchtest!{mutex_6t_082p, 6, 0.82}
benchtest!{mutex_6t_083p, 6, 0.83}
benchtest!{mutex_6t_084p, 6, 0.84}
benchtest!{mutex_6t_085p, 6, 0.85}
benchtest!{mutex_6t_086p, 6, 0.86}
benchtest!{mutex_6t_087p, 6, 0.87}
benchtest!{mutex_6t_088p, 6, 0.88}
benchtest!{mutex_6t_089p, 6, 0.89}
benchtest!{mutex_6t_090p, 6, 0.90}
benchtest!{mutex_6t_091p, 6, 0.91}
benchtest!{mutex_6t_092p, 6, 0.92}
benchtest!{mutex_6t_093p, 6, 0.93}
benchtest!{mutex_6t_094p, 6, 0.94}
benchtest!{mutex_6t_095p, 6, 0.95}
benchtest!{mutex_6t_096p, 6, 0.96}
benchtest!{mutex_6t_097p, 6, 0.97}
benchtest!{mutex_6t_098p, 6, 0.98}
benchtest!{mutex_6t_099p, 6, 0.99}
benchtest!{mutex_6t_100p, 6, 1.00}

benchtest!{mutex_7t_000p, 7, 0.00}
benchtest!{mutex_7t_001p, 7, 0.01}
benchtest!{mutex_7t_002p, 7, 0.02}
benchtest!{mutex_7t_003p, 7, 0.03}
benchtest!{mutex_7t_004p, 7, 0.04}
benchtest!{mutex_7t_005p, 7, 0.05}
benchtest!{mutex_7t_006p, 7, 0.06}
benchtest!{mutex_7t_007p, 7, 0.07}
benchtest!{mutex_7t_008p, 7, 0.08}
benchtest!{mutex_7t_009p, 7, 0.09}
benchtest!{mutex_7t_010p, 7, 0.10}
benchtest!{mutex_7t_011p, 7, 0.11}
benchtest!{mutex_7t_012p, 7, 0.12}
benchtest!{mutex_7t_013p, 7, 0.13}
benchtest!{mutex_7t_014p, 7, 0.14}
benchtest!{mutex_7t_015p, 7, 0.15}
benchtest!{mutex_7t_016p, 7, 0.16}
benchtest!{mutex_7t_017p, 7, 0.17}
benchtest!{mutex_7t_018p, 7, 0.18}
benchtest!{mutex_7t_019p, 7, 0.19}
benchtest!{mutex_7t_020p, 7, 0.20}
benchtest!{mutex_7t_021p, 7, 0.21}
benchtest!{mutex_7t_022p, 7, 0.22}
benchtest!{mutex_7t_023p, 7, 0.23}
benchtest!{mutex_7t_024p, 7, 0.24}
benchtest!{mutex_7t_025p, 7, 0.25}
benchtest!{mutex_7t_026p, 7, 0.26}
benchtest!{mutex_7t_027p, 7, 0.27}
benchtest!{mutex_7t_028p, 7, 0.28}
benchtest!{mutex_7t_029p, 7, 0.29}
benchtest!{mutex_7t_030p, 7, 0.30}
benchtest!{mutex_7t_031p, 7, 0.31}
benchtest!{mutex_7t_032p, 7, 0.32}
benchtest!{mutex_7t_033p, 7, 0.33}
benchtest!{mutex_7t_034p, 7, 0.34}
benchtest!{mutex_7t_035p, 7, 0.35}
benchtest!{mutex_7t_036p, 7, 0.36}
benchtest!{mutex_7t_037p, 7, 0.37}
benchtest!{mutex_7t_038p, 7, 0.38}
benchtest!{mutex_7t_039p, 7, 0.39}
benchtest!{mutex_7t_040p, 7, 0.40}
benchtest!{mutex_7t_041p, 7, 0.41}
benchtest!{mutex_7t_042p, 7, 0.42}
benchtest!{mutex_7t_043p, 7, 0.43}
benchtest!{mutex_7t_044p, 7, 0.44}
benchtest!{mutex_7t_045p, 7, 0.45}
benchtest!{mutex_7t_046p, 7, 0.46}
benchtest!{mutex_7t_047p, 7, 0.47}
benchtest!{mutex_7t_048p, 7, 0.48}
benchtest!{mutex_7t_049p, 7, 0.49}
benchtest!{mutex_7t_050p, 7, 0.50}
benchtest!{mutex_7t_051p, 7, 0.51}
benchtest!{mutex_7t_052p, 7, 0.52}
benchtest!{mutex_7t_053p, 7, 0.53}
benchtest!{mutex_7t_054p, 7, 0.54}
benchtest!{mutex_7t_055p, 7, 0.55}
benchtest!{mutex_7t_056p, 7, 0.56}
benchtest!{mutex_7t_057p, 7, 0.57}
benchtest!{mutex_7t_058p, 7, 0.58}
benchtest!{mutex_7t_059p, 7, 0.59}
benchtest!{mutex_7t_060p, 7, 0.60}
benchtest!{mutex_7t_061p, 7, 0.61}
benchtest!{mutex_7t_062p, 7, 0.62}
benchtest!{mutex_7t_063p, 7, 0.63}
benchtest!{mutex_7t_064p, 7, 0.64}
benchtest!{mutex_7t_065p, 7, 0.65}
benchtest!{mutex_7t_066p, 7, 0.66}
benchtest!{mutex_7t_067p, 7, 0.67}
benchtest!{mutex_7t_068p, 7, 0.68}
benchtest!{mutex_7t_069p, 7, 0.69}
benchtest!{mutex_7t_070p, 7, 0.70}
benchtest!{mutex_7t_071p, 7, 0.71}
benchtest!{mutex_7t_072p, 7, 0.72}
benchtest!{mutex_7t_073p, 7, 0.73}
benchtest!{mutex_7t_074p, 7, 0.74}
benchtest!{mutex_7t_075p, 7, 0.75}
benchtest!{mutex_7t_076p, 7, 0.76}
benchtest!{mutex_7t_077p, 7, 0.77}
benchtest!{mutex_7t_078p, 7, 0.78}
benchtest!{mutex_7t_079p, 7, 0.79}
benchtest!{mutex_7t_080p, 7, 0.80}
benchtest!{mutex_7t_081p, 7, 0.81}
benchtest!{mutex_7t_082p, 7, 0.82}
benchtest!{mutex_7t_083p, 7, 0.83}
benchtest!{mutex_7t_084p, 7, 0.84}
benchtest!{mutex_7t_085p, 7, 0.85}
benchtest!{mutex_7t_086p, 7, 0.86}
benchtest!{mutex_7t_087p, 7, 0.87}
benchtest!{mutex_7t_088p, 7, 0.88}
benchtest!{mutex_7t_089p, 7, 0.89}
benchtest!{mutex_7t_090p, 7, 0.90}
benchtest!{mutex_7t_091p, 7, 0.91}
benchtest!{mutex_7t_092p, 7, 0.92}
benchtest!{mutex_7t_093p, 7, 0.93}
benchtest!{mutex_7t_094p, 7, 0.94}
benchtest!{mutex_7t_095p, 7, 0.95}
benchtest!{mutex_7t_096p, 7, 0.96}
benchtest!{mutex_7t_097p, 7, 0.97}
benchtest!{mutex_7t_098p, 7, 0.98}
benchtest!{mutex_7t_099p, 7, 0.99}
benchtest!{mutex_7t_100p, 7, 1.00}

benchtest!{mutex_8t_000p, 8, 0.00}
benchtest!{mutex_8t_001p, 8, 0.01}
benchtest!{mutex_8t_002p, 8, 0.02}
benchtest!{mutex_8t_003p, 8, 0.03}
benchtest!{mutex_8t_004p, 8, 0.04}
benchtest!{mutex_8t_005p, 8, 0.05}
benchtest!{mutex_8t_006p, 8, 0.06}
benchtest!{mutex_8t_007p, 8, 0.07}
benchtest!{mutex_8t_008p, 8, 0.08}
benchtest!{mutex_8t_009p, 8, 0.09}
benchtest!{mutex_8t_010p, 8, 0.10}
benchtest!{mutex_8t_011p, 8, 0.11}
benchtest!{mutex_8t_012p, 8, 0.12}
benchtest!{mutex_8t_013p, 8, 0.13}
benchtest!{mutex_8t_014p, 8, 0.14}
benchtest!{mutex_8t_015p, 8, 0.15}
benchtest!{mutex_8t_016p, 8, 0.16}
benchtest!{mutex_8t_017p, 8, 0.17}
benchtest!{mutex_8t_018p, 8, 0.18}
benchtest!{mutex_8t_019p, 8, 0.19}
benchtest!{mutex_8t_020p, 8, 0.20}
benchtest!{mutex_8t_021p, 8, 0.21}
benchtest!{mutex_8t_022p, 8, 0.22}
benchtest!{mutex_8t_023p, 8, 0.23}
benchtest!{mutex_8t_024p, 8, 0.24}
benchtest!{mutex_8t_025p, 8, 0.25}
benchtest!{mutex_8t_026p, 8, 0.26}
benchtest!{mutex_8t_027p, 8, 0.27}
benchtest!{mutex_8t_028p, 8, 0.28}
benchtest!{mutex_8t_029p, 8, 0.29}
benchtest!{mutex_8t_030p, 8, 0.30}
benchtest!{mutex_8t_031p, 8, 0.31}
benchtest!{mutex_8t_032p, 8, 0.32}
benchtest!{mutex_8t_033p, 8, 0.33}
benchtest!{mutex_8t_034p, 8, 0.34}
benchtest!{mutex_8t_035p, 8, 0.35}
benchtest!{mutex_8t_036p, 8, 0.36}
benchtest!{mutex_8t_037p, 8, 0.37}
benchtest!{mutex_8t_038p, 8, 0.38}
benchtest!{mutex_8t_039p, 8, 0.39}
benchtest!{mutex_8t_040p, 8, 0.40}
benchtest!{mutex_8t_041p, 8, 0.41}
benchtest!{mutex_8t_042p, 8, 0.42}
benchtest!{mutex_8t_043p, 8, 0.43}
benchtest!{mutex_8t_044p, 8, 0.44}
benchtest!{mutex_8t_045p, 8, 0.45}
benchtest!{mutex_8t_046p, 8, 0.46}
benchtest!{mutex_8t_047p, 8, 0.47}
benchtest!{mutex_8t_048p, 8, 0.48}
benchtest!{mutex_8t_049p, 8, 0.49}
benchtest!{mutex_8t_050p, 8, 0.50}
benchtest!{mutex_8t_051p, 8, 0.51}
benchtest!{mutex_8t_052p, 8, 0.52}
benchtest!{mutex_8t_053p, 8, 0.53}
benchtest!{mutex_8t_054p, 8, 0.54}
benchtest!{mutex_8t_055p, 8, 0.55}
benchtest!{mutex_8t_056p, 8, 0.56}
benchtest!{mutex_8t_057p, 8, 0.57}
benchtest!{mutex_8t_058p, 8, 0.58}
benchtest!{mutex_8t_059p, 8, 0.59}
benchtest!{mutex_8t_060p, 8, 0.60}
benchtest!{mutex_8t_061p, 8, 0.61}
benchtest!{mutex_8t_062p, 8, 0.62}
benchtest!{mutex_8t_063p, 8, 0.63}
benchtest!{mutex_8t_064p, 8, 0.64}
benchtest!{mutex_8t_065p, 8, 0.65}
benchtest!{mutex_8t_066p, 8, 0.66}
benchtest!{mutex_8t_067p, 8, 0.67}
benchtest!{mutex_8t_068p, 8, 0.68}
benchtest!{mutex_8t_069p, 8, 0.69}
benchtest!{mutex_8t_070p, 8, 0.70}
benchtest!{mutex_8t_071p, 8, 0.71}
benchtest!{mutex_8t_072p, 8, 0.72}
benchtest!{mutex_8t_073p, 8, 0.73}
benchtest!{mutex_8t_074p, 8, 0.74}
benchtest!{mutex_8t_075p, 8, 0.75}
benchtest!{mutex_8t_076p, 8, 0.76}
benchtest!{mutex_8t_077p, 8, 0.77}
benchtest!{mutex_8t_078p, 8, 0.78}
benchtest!{mutex_8t_079p, 8, 0.79}
benchtest!{mutex_8t_080p, 8, 0.80}
benchtest!{mutex_8t_081p, 8, 0.81}
benchtest!{mutex_8t_082p, 8, 0.82}
benchtest!{mutex_8t_083p, 8, 0.83}
benchtest!{mutex_8t_084p, 8, 0.84}
benchtest!{mutex_8t_085p, 8, 0.85}
benchtest!{mutex_8t_086p, 8, 0.86}
benchtest!{mutex_8t_087p, 8, 0.87}
benchtest!{mutex_8t_088p, 8, 0.88}
benchtest!{mutex_8t_089p, 8, 0.89}
benchtest!{mutex_8t_090p, 8, 0.90}
benchtest!{mutex_8t_091p, 8, 0.91}
benchtest!{mutex_8t_092p, 8, 0.92}
benchtest!{mutex_8t_093p, 8, 0.93}
benchtest!{mutex_8t_094p, 8, 0.94}
benchtest!{mutex_8t_095p, 8, 0.95}
benchtest!{mutex_8t_096p, 8, 0.96}
benchtest!{mutex_8t_097p, 8, 0.97}
benchtest!{mutex_8t_098p, 8, 0.98}
benchtest!{mutex_8t_099p, 8, 0.99}
benchtest!{mutex_8t_100p, 8, 1.00}
