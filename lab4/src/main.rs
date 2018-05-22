#[macro_use]
extern crate clap;   // command line argument parser
#[macro_use]
extern crate log;    // logging macros

extern crate chrono; // time for logging
extern crate csv;    // csv writer for output data
extern crate fern;   // logging formatter

pub mod algorithms;
pub mod error;
pub mod util;

use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

use algorithms::*;
use error::Result;

fn simulate(input: Option<&str>, table_size: usize,
  to_table_size: Option<usize>, algorithm: &str) -> Result<Vec<(usize, f64)>> {

  // has to be here to prevent lock from going out of scope
  let stdin = io::stdin();

  // choose either a file or use stdin
  let reader = if let Some(file_name) = input {
    let file = File::open(file_name)?;
    info!("Reading page accesses from file {}", &file_name);
    // different reader types so return common BufRead trait object
    //  by allocating it on heap with Box<T>
    Box::new(BufReader::new(file)) as Box<BufRead>
  } else {
    info!("Reading page accesses from stdin");
    Box::new(stdin.lock()) as Box<BufRead>
  };

  // Vec of page requests from stdin or a file
  let mut page_requests = Vec::new();
  
  // read input from stdin or file to a vec first to allow for
  // repeat use for different memory sizes
  for line in reader.lines() {
    let line = line?;
    if let Ok(num) = line.parse::<u64>() {
      // only use positive numbers
      if num <= 0 {
        continue;
      }
      page_requests.push(num);
    }
  }

  let mut hit_rates = Vec::new();

  for i in table_size..=to_table_size.unwrap_or(table_size) {
    info!("Running simulation with table size {}", i);
    let mut page_table = match algorithm {
      "fifo" => AlgorithmType::Fifo(Fifo::new(i)),
      "lru" => AlgorithmType::Lru(Lru::new(i)),
      "second_chance" | "sc" => AlgorithmType::SecondChance(SecondChance::new(i)),
      _ => unreachable!(),
    };

    let mut num_requests = 0;
    let mut num_misses = 0;

    // iterate over input lines
    for &page_request in page_requests.iter() {
      num_requests += 1;

      // run corresponding page replacement algorithms
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
    debug!("Hits: {} / {}", num_hits, num_requests);
    println!("Hit rate: {:.5}",  hit_rate);

    hit_rates.push((i, hit_rate));
  }

  Ok(hit_rates)
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
