pub struct Lru {
  /// Vec of page numbers
  table: Vec<u64>,
  /// Size of page table
  size: usize,
}

impl Lru {
  /// Creates a new Lru page table
  pub fn new(size: usize) -> Self {
    info!("Using algorithm: LRU");
    Lru {
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

    // get index of item
    let index = self.table
      .iter()
      .position(|x| *x == page_request)
      .unwrap(); // can unwrap here since vec must contain the item here
    self.table.remove(index); // remove item

    // move page back to beginning
    self.table.push(page_request);

    debug!("{:?}", self.table);
    false
  }
}
