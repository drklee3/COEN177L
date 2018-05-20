#[derive (Debug)]
pub struct Lru {
  /// Vec of page numbers
  table: Vec<u64>,
  /// Size of page table
  size: usize,
  /// Index position
  index: usize,
}

impl Lru {
  pub fn new(size: usize) -> Self {
    Lru {
      table: Vec::with_capacity(size),
      size: size,
      index: 0,
    }
  }

  /// Handles a page request, returns true if page fault occurred
  pub fn handle_page_request(&mut self, page_request: u64) -> bool {
    if !self.table.contains(&page_request) {
      println!("Page {} caused a page fault", page_request);

      // we can't really use vec[i] past the number of elements even though
      // the _capacity_ is large enough. 

      // under capacity, just push to end
      if self.table.len() < self.size {
        self.table.push(page_request);
      } else {
        // not under capacity
        if let Some(elem) = self.table.get_mut(self.index) {
          trace!("Modified {} -> {} at index {}", *elem, page_request, self.index);
          *elem = page_request;
        }
      }

      // incrememnt circular "loop"
      self.index = (self.index + 1) % self.size;

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
