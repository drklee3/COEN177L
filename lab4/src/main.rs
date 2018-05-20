#[macro_use] 
extern crate log;    // logging macros
extern crate fern;   // logging formatter
#[macro_use]
extern crate clap;   // command line argument parser
extern crate chrono; // time for logging

use std::process;
use std::io::{self, BufRead};
use clap::{Arg, App};

pub mod util;
pub mod error;
pub mod algorithms;

use algorithms::*;

fn simulate(table_size: usize, algorithm: &str) {
  let mut page_table = match algorithm {
    "fifo" => AlgorithmType::Fifo(Fifo::new(table_size)),
    "lru" => AlgorithmType::Lru(Lru::new(table_size)),
    "second_chance" => AlgorithmType::SecondChance(SecondChance::new(table_size)),
    _ => return,
  };

  let mut page_request;
  let mut num_requests = 0;
  let mut num_misses = 0;

  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    let line = line.expect("Failed to read line from stdin");
    if let Ok(num) = line.parse::<u64>() {
      // only use positive numbers
      if num <= 0 {
        continue;
      }
      page_request = num;
    } else {
      continue;
    }

    num_requests += 1;

    // run page replacement algorithms
    let res = match page_table {
      AlgorithmType::Fifo(ref mut x) => x.handle_page_request(page_request),
      AlgorithmType::Lru(ref mut x) => x.handle_page_request(page_request),
      AlgorithmType::SecondChance(ref mut x) => x.handle_page_request(page_request),
    };
    
    // check if resulted in page fault
    if res {
      num_misses += 1;
    }
  }

  let num_hits = num_requests - num_misses;
  let hit_rate = num_hits as f64 / num_requests as f64;
  info!("Hit rate: {:.3}",  hit_rate);
}

fn main() {
  // parse args
  let args = App::new("page-replacements")
    .version(crate_version!())
    .author(crate_authors!())
    .about("Simulates various page replacement algorithms")
    .arg(Arg::with_name("table_size")
      .help("Sets the page table size")
      .required(true)
      .index(1)
      .validator(|size| {
        if let Ok(parsed) = size.parse::<usize>() {
          if parsed <= 0 {
            // don't think we can get negative numbers so this is
            // mainly just a check for 0
            return Err("Please give a number over 0".into());
          }
        } else {
          return Err("Please give a number".into());
        }

        Ok(())
      })
    )
    .arg(Arg::with_name("verbose")
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
  
  // parse table size
  let table_size = args
    .value_of("table_size")
    .and_then(|x| x.parse::<usize>().ok())
    .unwrap(); // ok to unwrap here, input already validated in clap
  
  let verbosity: u64 = args.occurrences_of("verbose");

  if let Err(e) = util::setup_logger(verbosity) {
    eprintln!("Error setting up logging: {}", e);
    process::exit(1);
  }

  // safe to unwrap, required & validated in clap
  let algorithm = args.value_of("algorithm").unwrap();

  info!("Using table size {}", table_size);
  info!("Using page replacement algorithm {}", algorithm.to_uppercase());
  simulate(table_size, algorithm);
}
