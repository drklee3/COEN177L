pub mod fifo;
pub mod lru;
pub mod second_chance;

// trait reexports
pub use fifo::Fifo;
pub use lru::Lru;
pub use second_chance::SecondChance;
pub use second_chance::Page;

pub enum AlgorithmType {
  Fifo,
  Lru,
  SecondChance,
}

pub struct PageTable<T> {
  /// Vec of page numbers
  table: Vec<T>,
  /// Size of page table
  size: usize,
}

impl<T> PageTable<T> {
  pub fn new(size: usize) -> Self {
    let table: Vec<T> = Vec::with_capacity(size);

    PageTable {
      table,
      size: size,
    }
  }
}
