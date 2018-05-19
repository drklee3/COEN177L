#[macro_use] 
extern crate log;
extern crate fern;
extern crate chrono;

use std::process;
use std::io::{self, BufRead};

pub mod util;
pub mod error;
pub mod algorithms;

fn main() {
  // unwrap used here to panic
  let table_size = match util::parse_args() {
    Ok(size) => size,
    Err(e) => {
      eprintln!("Error: {}", e);
      process::exit(1);
    }
  };
  util::setup_logger().expect("Failed to set up logging");

  info!("Using table size {}", table_size);

  let mut page_request;
  let mut num_requests = 0;
  let mut num_misses = 0;
  
  #[cfg(feature = "fifo")]
  let mut page_table = algorithms::fifo::Fifo::new(table_size);
  #[cfg(feature = "lru")]
  let mut page_table = algorithms::lru::Lru::new(table_size);
  #[cfg(feature = "second_chance")]
  let mut page_table = algorithms::second_chance::SecondChance::new(table_size);

  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    let line = line.expect("Failed to read line from stdin");
    if let Ok(num) = line.parse::<u64>() {
      // check if 0
      page_request = num;
    } else {
      continue;
    }
    num_requests += 1;
    
    
    if page_table.handle_page_request(page_request) {
      num_misses += 1;
    }
  }

  let num_hits = num_requests - num_misses;
  let hit_rate = num_hits as f64 / num_requests as f64;
  info!("Hit rate: {:.3}",  hit_rate);
}
