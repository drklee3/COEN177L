pub mod fifo;
pub mod lru;
pub mod second_chance;

// struct reexports
pub use self::fifo::Fifo;
pub use self::lru::Lru;
pub use self::second_chance::SecondChance;

/// Enum to hold the different types of page replacement algorithms
pub enum AlgorithmType {
  Fifo(Fifo),
  Lru(Lru),
  SecondChance(SecondChance),
}

pub struct Simulation {
  algorithm: AlgorithmType,
  num_requests: u64,
  num_misses: u64,
}

impl Simulation {
  pub fn new(table_size: usize, algorithm: &str) -> Self {
    let algorithm = match algorithm {
      "fifo" => AlgorithmType::Fifo(Fifo::new(table_size)),
      "lru" => AlgorithmType::Lru(Lru::new(table_size)),
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
      AlgorithmType::SecondChance(ref mut x) => x.handle_page_request(page_request, should_stdout),
    };
    
    // check if resulted in page fault
    if res {
      self.num_misses += 1;
    }
  }

  pub fn get_hit_rate(&self, ) -> f64 {
    let num_hits = self.num_requests - self.num_misses;
    let hit_rate = num_hits as f64 / self.num_requests as f64;
    debug!("Hits: {} / {}", num_hits, self.num_requests);
    println!("Hit rate: {:.5}",  hit_rate);

    hit_rate
  }
}

/*
// Could use generic struct but conditional traits are too hard idk
pub struct PageTable<T> {
  /// Vec of page numbers
  table: Vec<T>,
  /// Size of page table
  size: usize,
}
*/