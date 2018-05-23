use std::fmt;

/// A page table entry for LRU page replacement
#[derive (Clone)]
pub struct LruPage {
  /// Page number
  number: u64,
  /// "Time" of insertion
  time: u64,
}

impl LruPage {
  /// Creates a new LRU page
  fn new() -> Self {
    LruPage {
      number: 0,
      time: 0,
    }
  }
}

impl fmt::Debug for LruPage {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} ({})", self.number, self.time)
  }
}

/// A page table for LRU page replacement
#[derive (Debug)]
pub struct Lru {
  /// Vec of page numbers
  table: Vec<LruPage>,
  /// Size of page table
  size: usize,
  /// Current "time"
  time: u64,
}

/// A helper struct to store page table min/max values/indexes
pub struct PageMinMax {
  /// Max "time" value of a page
  min: u64,
  /// Min "time" value of a page
  max: u64,

  /// Index of min time page
  min_index: usize,
  /// Index of max time page
  max_index: usize,
}

impl PageMinMax {
  /// Creates a new PageMinMax
  fn new() -> Self {
    PageMinMax {
      min: <u64>::max_value(),
      max: 0,
      min_index: 0,
      max_index: 0,
    }
  }
}

impl Lru {
  /// Creates a new page table for LRU
  pub fn new(size: usize) -> Self {
    Lru {
      table: vec![LruPage::new(); size],
      size: size,
      time: 0,
    }
  }

  /// Handles a page request, returns true if page fault occurred
  pub fn handle_page_request(&mut self, page_request: u64, should_stdout: bool) -> bool {
    // get min and max times and indexes
    let min_max_page = self.table
      .iter()
      .enumerate()
      .fold(PageMinMax::new(), |mut acc, (i, x)| {
        if x.time < acc.min {
          acc.min = x.time;
          acc.min_index = i;
        }
        
        if x.time > acc.max {
          acc.max = x.time;
          acc.max_index = i;
        }

        acc
      });
    
    // do this before check so don't need 2 searches to find this
    // entry again if it does exist
    let page_index = self.table
      .iter()
      .position(|x| x.number == page_request);

    // check if in memory / page table
    if page_index.is_none() {
      if should_stdout {
        println!("Page {} caused a page fault", page_request);
      }

      // create a new page entry
      let new_page = LruPage {
        number: page_request,
        time: min_max_page.max + 1,
      };
      
      // replace oldest entry with new one
      {
        // mutable borrow
        let elem = self.table.get_mut(min_max_page.min_index).unwrap();
        trace!("SWAP: [{:?} -> {:?}] @ i = {}", *elem, &new_page, min_max_page.min_index);
        *elem = new_page;
        // mutable borrow ends
      }

      debug!("{:?}", self.table);
      return true;
    }

    // update time for existing page
    {
      // can unwrap here since vec must contain the item here
      let index = page_index.unwrap();
      let elem = self.table.get_mut(index).unwrap();
      trace!("ADJUST: #{} time [{} -> {}] @ i = {}",
        elem.number, elem.time, min_max_page.max + 1, index);
      elem.time = min_max_page.max + 1;
    }
    
    debug!("{:?}", self.table);
    false
  }
}
