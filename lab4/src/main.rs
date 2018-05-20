#[macro_use] 
extern crate log;    // logging macros
extern crate fern;   // logging formatter
#[macro_use]
extern crate clap;   // command line argument parser
extern crate chrono; // time for logging

use std::process;
use std::io::{self, BufRead};
use clap::{Arg, App, SubCommand};

pub mod util;
pub mod error;
pub mod algorithms;

use algorithms::*;

fn simulate(table_size: usize, algorithm: &str) {
  let mut page_table = match algorithm {
    "fifo" => PageTable::new(table_size),
    "lru" => PageTable::new(table_size),
    "second_chance" => SecondChance::new(table_size),
    _ => return,
  };

  let mut page_request;
  let mut num_requests = 0;
  let mut num_misses = 0;

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

    let res = match algorithm {
      "fifo" => Fifo::handle_page_request(&mut page_table, page_request),
      "lru" => Lru::handle_page_request(&mut page_table, page_request),
      "second_chance" => SecondChance::handle_page_request(&mut page_table, page_request),
      _ => return,
    };
    
    if res {
      num_misses += 1;
    }
  }

  let num_hits = num_requests - num_misses;
  let hit_rate = num_hits as f64 / num_requests as f64;
  info!("Hit rate: {:.3}",  hit_rate);
}

fn main() {
  let matches = App::new("page-replacements")
    .version(crate_version!())
    .author(crate_authors!())
    .about("Simulates various page replacement algorithms")
    .arg(Arg::with_name("table_size")
      .help("Sets the page table size")
      .required(true)
      .index(1)
    )
    .arg(Arg::with_name("v")
      .short("v")
      .multiple(true)
      .help("Sets the level of verbosity")
    )
    .arg(Arg::with_name("algorithm")
      .short("a")
      .long("algorithm")
      .help("Sets the page replacement algorithm to use")
      .required(true)
      .takes_value(true)
      .possible_values(&["fifo", "lru", "second_chance"])
    )
    .get_matches();

  let table_size = match matches
    .value_of("table_size")
    .and_then(|x| x.parse::<usize>().ok()) {
    Some(size) => size,
    None => {
      eprintln!("Error: invalid table size");
      process::exit(1);
    }
  };

  if let Err(e) = util::setup_logger() {
    eprintln!("Error setting up logging: {}", e);
    process::exit(1);
  }

  info!("Using table size {}", table_size);

  let algorithm = match matches.value_of("algorithm") {
    Some(alg) => alg,
    None => {
      eprintln!("Invalid algorithm");
      process::exit(1);
    }
  };

  // Conditional compilations based on which paging algorithm feature is enabled
  #[cfg(feature = "fifo")]
  let mut page_table = algorithms::PageTable::new(table_size);
  #[cfg(feature = "lru")]
  let mut page_table = lru::Lru::new(table_size);
  #[cfg(feature = "second_chance")]
  let mut page_table = second_chance::SecondChance::new(table_size);

  simulate(table_size, algorithm);
}
