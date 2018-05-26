use parking_lot::RwLock;
use std::sync::Arc;
use std::process;

// This optimal implementation is **not** working correctly
// was intended to be used to compare with the other algorithms

/// A page table entry for Optimal page replacement
#[derive (Debug)]
pub struct Optimal {
  /// Vec of page numbers
  table: Vec<u64>,
  /// Full future page requests
  page_requests: Arc<RwLock<Vec<String>>>,
  /// Size of page table
  size: usize,
  /// Position of future page requests
  index: usize,
}

impl Optimal {
  pub fn new(size: usize, page_requests: Option<Arc<RwLock<Vec<String>>>>) -> Self {
    if page_requests.is_none() {
      error!("Must run optimal with page requests as an input file");
      process::exit(1);
    }

    Optimal {
      table: vec![0; size], // initialize vec with size 0s
      page_requests: page_requests.unwrap(),
      size,
      index: 0,
    }
  }

  /// Handles a page request, returns true if page fault occurred
  pub fn handle_page_request(&mut self, page_request: u64, should_stdout: bool) -> bool {
    // increment position in "future" page requests
    self.index += 1;

    if !self.table.contains(&page_request) {
      if should_stdout {
        println!("Page {} caused a page fault", page_request);
      }

      // search for any pages with number 0 (all initialized to 0)
      let free_page_index = self.table
        .iter()
        .position(|&x| x == 0);

      // contains pages with 0
      // means not full since initialized to 0 and does not accept #0
      if let Some(index) = free_page_index {
        let elem = self.table.get_mut(index).unwrap();
        trace!("Added page {}", page_request);
        *elem = page_request;
      } else {
        // table is full
        // search for furthest page
        let mut furthest_distance = 0;
        let mut furthest_index = 0;
        let page_requests_slice = &self.page_requests.read()[self.index..];
        for (i, page) in self.table.iter().enumerate() {
          // no pages should be 0 since full, don't have to check
          // iterate over future page requests from current position
          // to find the furthest distance
          let distance = page_requests_slice
            .iter()
            .position(|future_page| future_page
              .parse::<u64>()
              .unwrap_or(0) == *page
            );

          if let Some(distance) = distance {
            // check if this is furthest page
            if distance > furthest_distance {
              furthest_distance = distance;
              furthest_index = i;
            }
          } else {
            // cannot find in future requests,
            // won't ever be used again so replace this one
            furthest_index = i;
            break;
          }
        }

        // replace page with furthest
        let furthest_elem = self.table.get_mut(furthest_index).unwrap();
        trace!("Replaced {} -> {}", *furthest_elem, page_request);
        *furthest_elem = page_request;
      }

      debug!("{:?}", self.table);
      return true;
    }
    debug!("{:?}", self.table);

    false
  }
}
