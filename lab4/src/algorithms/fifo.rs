#[derive (Debug)]
pub struct Fifo {
  /// Vec of page numbers
  table: Vec<u64>,
  /// Size of page table
  size: usize,
}

impl Fifo {
  pub fn new(size: usize) -> Self {
    Fifo {
      table: Vec::with_capacity(size),
      size: size,
    }
  }

  /// Handles a page request, returns true if page fault occurred
  pub fn handle_page_request(&mut self, page_request: u64) -> bool {
    if !self.table.contains(&page_request) {
      println!("Page fault: {}", page_request);

      // remove first page if over capacity
      if self.table.len() >= self.size {
        self.table.remove(0);
      }

      // push new page request
      self.table.push(page_request);
      debug!("{:?}", self.table);
      return true;
    }

    // we don't move existing page to beginning
    // since that would be basically just lru?
    debug!("{:?}", self.table);
    false
  }
}
