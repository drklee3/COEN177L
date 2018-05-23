use algorithms::*;
use error::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::Arc;
use parking_lot::{Mutex, RwLock};
use threadpool::Builder;
use model::simulation::*;

/// Runs a simulation or simulations for a range of table sizes,
/// buffers input via a file given to allow for page request input reuse
pub fn simulate_file(options: SimulationOptions) -> Result<Vec<(usize, f64)>> {

  let file_name = options.input.unwrap(); // checked before
  let table_size = options.table_size;
  let to_table_size = options.to_table_size;
  let algorithm = options.algorithm;
  let should_stdout = options.should_stdout;

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

  let page_requests = Arc::new(RwLock::new(page_requests));

  // thread safe hit rates
  // mutex in atomically referenced counted pointer
  let hit_rates = Arc::new(Mutex::new(Vec::new()));

  // build threadpool, # threads = cpu count
  let pool = Builder::new()
    .thread_name("simulation_worker".into())
    .build();
  
  info!("Using {} threads for concurrent simulations", pool.max_count());

  // repeat for table size range
  for curr_table_size in table_size..=to_table_size.unwrap_or(table_size) {
    // clone hit_rate vec pointer
    let page_requests = page_requests.clone();
    let hit_rates = hit_rates.clone();
    let algorithm = algorithm.to_string();
    // run on threadpool
    pool.execute(move || {
      info!("Running simulation with table size {}", curr_table_size);
      let mut sim = Simulation::new(curr_table_size, &algorithm);
      // iterate over file lines
      let reader = page_requests.read();
      for page_request in reader.iter() {
        sim.page_request(page_request, should_stdout);
      }

      {
        // push hit rate to vec
        let mut guard = hit_rates.lock();
        guard.push((curr_table_size, sim.get_hit_rate()));
      }
    })
  }

  // wait until jobs finished
  pool.join();

  Ok(Arc::try_unwrap(hit_rates).unwrap().into_inner().clone())
}

/// Runs a single simulation without input buffering to allow for immediate
/// feedback per page request, main use case for testing
pub fn simulate_stdin(table_size: usize, algorithm: &str, should_stdout: bool) -> Result<f64> {
  let mut sim = Simulation::new(table_size, algorithm);
  let stdin = io::stdin();

  // iterate over input lines
  for line in stdin.lock().lines() {
    let page_request = line?;
    sim.page_request(&page_request, should_stdout);
  }

  Ok(sim.get_hit_rate())
}

/// Checks if there is an input file and runs simulations,
/// uses stdin input if no input file found
pub fn simulate(options: SimulationOptions) -> Result<Vec<(usize, f64)>> {
  if options.input.is_some() {
    let hit_rates = simulate_file(options);

    return hit_rates;
  }

  // no input file, read from stdin
  let mut hit_rates = Vec::new();
  let hit_rate = simulate_stdin(options.table_size,
    options.algorithm, options.should_stdout);
  hit_rates.push((options.table_size, hit_rate.unwrap()));

  Ok(hit_rates)
}
