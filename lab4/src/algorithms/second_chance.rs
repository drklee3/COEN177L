use std::fmt;

/// A page table entry for second chance page replacement
#[derive (Clone)]
pub struct SecondChancePage {
  /// Page number
  number: u64,
  /// Referenced "bit"
  referenced: bool,
}

impl SecondChancePage {
  fn new() -> Self {
    SecondChancePage {
      number: 0,
      referenced: false,
    }
  }
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
  /// Index position
  index: usize,
}

impl SecondChance {
  pub fn new(size: usize) -> Self {
    SecondChance {
      table: vec![SecondChancePage::new(); size],
      size: size,
      index: 0,      
    }
  }

  /// Handles a page request, returns true if page fault occurred
  pub fn handle_page_request(&mut self, page_request: u64, should_stdout: bool) -> bool {
    // get page index, returns None if doesn't exist in table
    let page_index = self.table
      .iter()
      .position(|x| x.number == page_request);
    
    // doesn't contain page number, not in memory
    if page_index.is_none() {
      // big bottleneck!!
      if should_stdout {
        println!("Page {} caused a page fault", page_request);
      }

      loop {
        // safe to unwrap, vec is initialized to be full and self.index is circular
        let page = self.table
          .get_mut(self.index)
          .unwrap();
        
        // move to next page, circular loop
        self.index = (self.index + 1) % self.size;
        
        if !page.referenced {
          // replace page with new page request #, referenced should still be false
          trace!("Replace entry: {} -> {}", page.number, page_request);
          page.number = page_request;
          break;
        }

        trace!("Reset entry: {}", page.number);
        page.referenced = false;
      }

      debug!("{:?}", self.table);
      return true;
    }
    // self.index = (self.index + 1) % self.size;

    // safe to unwrap here, returns before if None
    let index = page_index.unwrap();

    // update page to be referenced
    if let Some(page) = self.table.get_mut(index) {
      trace!("Update entry: {:?}", page);
      page.referenced = true;
    } else {
      error!("Failed to get table value @ {}", index);
    }

    debug!("{:?}", self.table);
    false
  }
}
