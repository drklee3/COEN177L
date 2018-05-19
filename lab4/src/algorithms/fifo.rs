pub struct Fifo {
  /// Vec of page numbers
  table: Vec<u64>,
  /// Size of page table
  size: usize,
}

impl Fifo {
  /// Creates a new Fifo page table
  pub fn new(size: usize) -> Self {
    Fifo {
      table: Vec::with_capacity(size),
      size: size,
    }
  }

  pub fn handle_page_request(&mut self, page_request: u64) -> bool {
    debug!("{:?}", self.table);
    if !self.table.contains(&page_request) {
      println!("Page {} caused a page fault", page_request);

      // remove first page if over capacity
      if self.table.len() >= self.size {
        self.table.remove(0);
      }

      // push new page request
      self.table.push(page_request);

      return true;
    }

    false
  }
}
