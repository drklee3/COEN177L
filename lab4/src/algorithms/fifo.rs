use algorithms::PageTable;

pub trait Fifo {
  fn new(size: usize) -> Self;  
  fn handle_page_request(&mut self, page_request: u64) -> bool;
}

impl Fifo for PageTable<u64> {
  fn new(size: usize) -> Self {
    let table: Vec<u64> = Vec::with_capacity(size);
    PageTable {
      table,
      size: size,
    }
  }

  /// Handles a page request, returns true if page fault occurred
  fn handle_page_request(&mut self, page_request: u64) -> bool {
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
