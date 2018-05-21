use std::fmt;

/// A page table entry for second chance page replacement
#[derive (Clone)]
pub struct SecondChancePage {
  /// Page number
  number: u64,
  /// Referenced "bit"
  referenced: bool,
}

impl fmt::Debug for SecondChancePage {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.referenced {
      // blue number referenced
      write!(f, "\x1b[0;36m{}\x1b[0;0m", self.number)
    } else {
      // red number unreferenced
      write!(f, "\x1b[0;31m{}\x1b[0;0m", self.number)
    }
  }
}

/// A page table for second chance page replacement
#[derive (Debug)]
pub struct SecondChance {
  /// Vec of page numbers
  table: Vec<SecondChancePage>,
  /// Size of page table
  size: usize,
}

impl SecondChance {
  pub fn new(size: usize) -> Self {
    SecondChance {
      table: Vec::with_capacity(size),
      size: size,
    }
  }

  /// Handles a page request, returns true if page fault occurred
  pub fn handle_page_request(&mut self, page_request: u64) -> bool {
    // search if contains, use any() to search with struct field
    if !self.table.iter().any(|x| x.number == page_request) {
      println!("Page {} caused a page fault", page_request);

      // remove first page if over capacity
      if self.table.len() >= self.size {
        // cannot use iterator over self.table here
        loop {
          // safe to unwrap, len has to be at least 1
          let mut page = self.table
            .first()
            .unwrap()
            .clone();
          
          // not referenced, can throw out
          if !page.referenced {
            // safe to unwrap, this only runs when table @ max capacity
            trace!("Removed entry: {:?}", self.table.first().unwrap());
            self.table.remove(0);
            // removed a page, can exit loop
            break;
          }

          trace!("Reset entry: {:?}", self.table.first().unwrap());
          // page is referenced, reset referenced and move to end of list
          self.table.remove(0);
          page.referenced = false;
          self.table.push(page);
        }
      }
    

      // push new page request
      let new_page = SecondChancePage {
        number: page_request,
        referenced: false,
      };

      self.table.push(new_page);
      debug!("{:?}", self.table);
      return true;
    }

    // get page index
    let index = self.table
      .iter()
      .position(|x| x.number == page_request)
      .unwrap();

    // update page to be referenced
    if let Some(page) = self.table.get_mut(index) {
      trace!("Update entry: {:?}", page);
      page.referenced = true;
    }

    debug!("{:?}", self.table);
    false
  }
}
