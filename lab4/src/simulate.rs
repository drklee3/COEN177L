use std::fs::File;
use std::io::{self, BufRead, BufReader};
use algorithms::*;
use error::Result;

/// Runs a simulation or simulations for a range of table sizes,
/// buffers input via a file given to allow for page request input reuse
pub fn simulate_file(file_name: &str, table_size: usize,
  to_table_size: Option<usize>, algorithm: &str) -> Result<Vec<(usize, f64)>> {
  let file = File::open(file_name)?;
  info!("Reading page accesses from file {}", &file_name);
  let reader = BufReader::new(file);

  // Vec of page requests from file
  let mut page_requests = Vec::new();
  
  // read input from file to a vec first to allow for
  // repeat use for different memory sizes
  for line in reader.lines() {
    let line = line?;
    page_requests.push(line);
  }

  let mut hit_rates = Vec::new();

  // repeat for table size range
  for curr_table_size in table_size..=to_table_size.unwrap_or(table_size) {
    let mut sim = Simulation::new(curr_table_size, algorithm);
    // iterate over file lines
    for page_request in &page_requests {
      sim.page_request(page_request);
    }

    // push hit rate to vec
    hit_rates.push((curr_table_size, sim.get_hit_rate()));
  }

  Ok(hit_rates)
}

/// Runs a single simulation without input buffering to allow for immediate
/// feedback per page request, main use case for testing
pub fn simulate_stdin(table_size: usize, algorithm: &str) -> Result<f64> {
  info!("Running simulation with table size {}", table_size);
  let mut sim = Simulation::new(table_size, algorithm);
  let stdin = io::stdin();

  // iterate over input lines
  for line in stdin.lock().lines() {
    let page_request = line?;
    sim.page_request(&page_request);
  }

  Ok(sim.get_hit_rate())
}

/// Checks if there is an input file and runs simulations,
/// uses stdin input if no input file found
pub fn simulate(input: Option<&str>, table_size: usize,
  to_table_size: Option<usize>, algorithm: &str) -> Result<Vec<(usize, f64)>> {
  
  if let Some(file_name) = input {
    let hit_rates = simulate_file(file_name, table_size, to_table_size, algorithm);

    return hit_rates;
  }

  // no input file, read from stdin
  let mut hit_rates = Vec::new();
  let hit_rate = simulate_stdin(table_size, algorithm);
  hit_rates.push((table_size, hit_rate.unwrap()));

  Ok(hit_rates)
}
