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
      table: vec![0; size],
      size: size,
      index: 0,
    }
  }

  /// Handles a page request, returns true if page fault occurred
  pub fn handle_page_request(&mut self, page_request: u64) -> bool {
    if !self.table.contains(&page_request) {
      println!("Page {} caused a page fault", page_request);

      if let Some(elem) = self.table.get_mut(self.index) {
        trace!("Modified {} -> {} at index {}", *elem, page_request, self.index);
        *elem = page_request;
      } else {
        unreachable!();
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
