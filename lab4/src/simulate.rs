use error::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::Arc;
use parking_lot::{Mutex, RwLock};
use threadpool::Builder;
use model::simulation::*;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};

/// Runs a simulation or simulations for a range of table sizes,
/// buffers input via a file given to allow for page request input reuse
pub fn simulate_file(options: SimulationOptions) -> Result<Vec<(usize, f64)>> {

  // destructure options struct
  let SimulationOptions {
    input,
    table_size,
    to_table_size,
    algorithm,
    should_stdout,
  } = options;

  let file_name = input.unwrap(); // checked before so ok to unwrap
  // use to table size or just use same as table_size
  let to_table_size = to_table_size.unwrap_or(table_size);

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

  // vec of page requests, same content as the input file
  // not really necessary to use a rwlock since we are not
  // modifying the vec after reading the file above
  let page_requests = Arc::new(RwLock::new(page_requests));

  // thread safe hit rates
  // mutex in atomically referenced counted pointer
  let hit_rates = Arc::new(Mutex::new(Vec::new()));

  // build threadpool, # threads = cpu count
  let pool = Builder::new()
    .thread_name("simulation_worker".into())
    .build();
  
  info!("Using {} threads for concurrent simulations", pool.max_count());

  let num_simulations = if to_table_size != table_size {
    (to_table_size - table_size) as u64
  } else {
    1
  };

  let progress_bar = ProgressBar::new(num_simulations);
  let sty = ProgressStyle::default_bar()
    .template("[{elapsed_precise}] ETA {eta} {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
    .progress_chars("##-");

  progress_bar.set_draw_target(ProgressDrawTarget::stderr());
  progress_bar.set_style(sty);

  let bar = Arc::new(progress_bar);

  // repeat for table size range
  for curr_table_size in table_size..=to_table_size {
    // clone hit_rate vec pointer
    let page_requests = page_requests.clone();
    let hit_rates = hit_rates.clone();
    let algorithm = algorithm.to_string();
    let bar = bar.clone();
    // run on threadpool
    pool.execute(move || {
      // info!("Running simulation with table size {}", curr_table_size);
      bar.set_message(&format!("Simulating table size {}", curr_table_size));
      let mut sim = Simulation::new(curr_table_size, &algorithm);
      // iterate over file lines
      let reader = page_requests.read();
      for page_request in reader.iter() {
        sim.page_request(page_request, should_stdout);
      }

      {
        // push hit rate to vec
        let mut guard = hit_rates.lock();
        guard.push((curr_table_size, sim.get_hit_rate(should_stdout)));
      }
      bar.inc(1);
    })
  }

  // wait until jobs finished
  pool.join();
  bar.finish_with_message(&format!("Finished {} simulations", num_simulations));

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

  Ok(sim.get_hit_rate(should_stdout))
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
