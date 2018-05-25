/// A page table entry for FIFO page replacement
#[derive (Debug)]
pub struct Fifo {
  /// Vec of page numbers
  table: Vec<u64>,
  /// Size of page table
  size: usize,
  /// Index position
  index: usize,
}

impl Fifo {
  pub fn new(size: usize) -> Self {
    Fifo {
      table: vec![0; size], // initialize vec with size 0s
      size: size,
      index: 0,
    }
  }

  /// Handles a page request, returns true if page fault occurred
  pub fn handle_page_request(&mut self, page_request: u64, should_stdout: bool) -> bool {
    if !self.table.contains(&page_request) {
      if should_stdout {
        println!("Page {} caused a page fault", page_request);
      }

      // safe to unwrap, self.index should never go >= len 
      {
        let elem = self.table.get_mut(self.index).unwrap();
        trace!("SWAP: {} -> {} at index {}", *elem, page_request, self.index);
        *elem = page_request;
      }

      self.index = (self.index + 1) % self.size;

      debug!("{:?}", self.table);
      return true;
    }

    // we don't move existing page to beginning
    // since that would be basically just lru?
    debug!("{:?}", self.table);
    false
  }
}
