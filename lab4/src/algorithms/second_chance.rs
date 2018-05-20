use algorithms::PageTable;

#[derive (Debug, Clone)]
pub struct Page {
  /// Page number
  number: u64,
  /// Referenced "bit"
  referenced: bool,
}

/*
pub struct SecondChance {
  /// Vec of page numbers
  table: Vec<Page>,
  /// Size of page table
  size: usize,
}
*/

pub trait SecondChance {
  fn new(size: usize) -> Self;
  fn handle_page_request(&mut self, page_request: u64) -> bool;
}

impl SecondChance for PageTable<u64> {
  fn new(size: usize) -> Self {
    let table: Vec<Page> = Vec::with_capacity(size);
    PageTable {
      table,
      size: size,
    }
  }

  /// Handles a page request, returns true if page fault occurred
  fn handle_page_request(&mut self, page_request: u64) -> bool {
    // search if contains, use any() to search with struct field
    if !self.table.iter().any(|x| x.number == page_request) {
      println!("Page fault: {}", page_request);

      // remove first page if over capacity
      if self.table.len() >= self.size {
        let mut removed = false;
        // cannot use iterator over self.table here
        for _ in 0..self.table.len() {
          // safe to unwrap, len has to be at least 1
          let mut page = self.table
            .first()
            .unwrap()
            .clone();
          
          // not referenced, can throw out
          if !page.referenced {
            self.table.remove(0);
            removed = true;
            break;
          }

          // page is referenced, reset referenced and move to end of list
          self.table.remove(0);
          page.referenced = false;
          self.table.push(page);
        }

        // all pages referenced, just remove last one
        if !removed {
          self.table.remove(0);
        }
      }
    

      // push new page request
      let new_page = Page {
        number: page_request,
        referenced: false,
      };

      self.table.push(new_page);
      debug!("{:#?}", self.table);
      return true;
    }

    // get page index
    let index = self.table
      .iter()
      .position(|x| x.number == page_request)
      .unwrap();

    // update page to be referenced
    if let Some(page) = self.table.get_mut(index) {
      page.referenced = true;
    }

    debug!("{:#?}", self.table);
    false
  }
}
