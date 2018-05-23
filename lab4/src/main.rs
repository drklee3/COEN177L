#[macro_use]
extern crate clap;   // command line argument parser
#[macro_use]
extern crate log;    // logging macros

extern crate chrono; // time for logging
extern crate csv;    // csv writer for output data
extern crate fern;   // logging formatter

pub mod algorithms;
pub mod error;
pub mod simulate;
pub mod util;

use clap::{App, Arg};
use std::process;

use simulate::*;


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
      .validator(util::validate_table_size)
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
      .possible_values(&["fifo", "lru", "second_chance", "sc"])
    )
    .arg(Arg::with_name("to_table_size")
      .short("t")
      .long("to")
      .help("Sets the max page table size to test a range of sizes")
      .takes_value(true)
      .validator(util::validate_table_size)
    )
    .arg(Arg::with_name("input")
      .short("i")
      .short("input")
      .help("Input file for page file access numbers")
      .takes_value(true)
    )
    .arg(Arg::with_name("output")
      .short("o")
      .long("output")
      .help("Sets the output csv file to write results to")
      .takes_value(true)
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

  // optional file input
  let input = args.value_of("input");

  let to_table_size = args
    .value_of("to_table_size")
    .and_then(|x| x.parse::<usize>().ok());

  // check if testing a range of page table sizes
  if let Some(size_to) = to_table_size {
    if size_to < table_size {
      error!("Max table size (-t size) cannot be lower than table size");
      process::exit(1);
    }
    info!("Using page replacement algorithm {} for table sizes {} -> {}",
      algorithm.to_uppercase(), table_size, size_to);
  } else {
    info!("Using page replacement algorithm {} for table size {}",
      algorithm.to_uppercase(), table_size);
  }
  
  // run simulation(s)
  let hit_rates = match simulate(input, table_size, to_table_size, algorithm) {
    Ok(rates) => rates,
    Err(e) => {
      error!("Failed simulation: {}", e);
      process::exit(1);
    }
  };

  // save hit rates to csv file
  if let Some(output_file) = args.value_of("output") {
    if let Err(e) = util::save_result(output_file, algorithm, hit_rates) {
      error!("Failed to save results: {}", e);
    }
  }
}
