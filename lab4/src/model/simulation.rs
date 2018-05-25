use model::algorithms::*;
use parking_lot::RwLock;
use std::sync::Arc;

pub struct SimulationOptions<'a> {
  pub input: Option<&'a str>,
  pub table_size: usize,
  pub to_table_size: Option<usize>,
  pub algorithm: &'a str,
  pub should_stdout: bool,
}

pub struct Simulation {
  algorithm: AlgorithmType,
  num_requests: u64,
  num_misses: u64,
}

impl Simulation {
  pub fn new(table_size: usize, algorithm: &str,
    page_requests: Option<Arc<RwLock<Vec<String>>>>) -> Self {
    let algorithm = match algorithm {
      "fifo" => AlgorithmType::Fifo(Fifo::new(table_size)),
      "lru" => AlgorithmType::Lru(Lru::new(table_size)),
      "optimal" => AlgorithmType::Optimal(Optimal::new(table_size, page_requests)),
      "second_chance" | "sc" => AlgorithmType::SecondChance(SecondChance::new(table_size)),
      _ => unreachable!(),
    };

    Simulation {
      algorithm,
      num_requests: 0,
      num_misses: 0,
    }
  }

  pub fn parse_line(&self, line: &str) -> Option<u64> {
    line.parse::<u64>()
      .ok()
      .and_then(|num| {
        if num <= 0 {
          None
        } else {
          Some(num)
        }
      })
  }

  pub fn page_request(&mut self, page_request: &str, should_stdout: bool) {
    let page_request = match self.parse_line(page_request) {
      Some(req) => req,
      None => return,
    };

    self.num_requests += 1;

    // run corresponding page replacement algorithms
    let res = match self.algorithm {
      AlgorithmType::Fifo(ref mut x) => x.handle_page_request(page_request, should_stdout),
      AlgorithmType::Lru(ref mut x) => x.handle_page_request(page_request, should_stdout),
      AlgorithmType::Optimal(ref mut x) => x.handle_page_request(page_request, should_stdout),
      AlgorithmType::SecondChance(ref mut x) => x.handle_page_request(page_request, should_stdout),
    };
    
    // check if resulted in page fault
    if res {
      self.num_misses += 1;
    }
  }

  pub fn get_hit_rate(&self, should_stdout: bool) -> f64 {
    let num_hits = self.num_requests - self.num_misses;
    let hit_rate = num_hits as f64 / self.num_requests as f64;
    debug!("Hits: {} / {}", num_hits, self.num_requests);
    if should_stdout {
      println!("Hit rate: {:.5}",  hit_rate);
    }

    hit_rate
  }
}
