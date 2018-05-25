pub mod fifo;
pub mod lru;
pub mod optimal;
pub mod second_chance;

// struct reexports
pub use self::fifo::Fifo;
pub use self::lru::Lru;
pub use self::optimal::Optimal;
pub use self::second_chance::SecondChance;

/// Enum to hold the different types of page replacement algorithms
pub enum AlgorithmType {
  Fifo(Fifo),
  Lru(Lru),
  Optimal(Optimal),
  SecondChance(SecondChance),
}

/*
// Could use generic struct but conditional traits are too hard idk
pub struct PageTable<T> {
  /// Vec of page numbers
  table: Vec<T>,
  /// Size of page table
  size: usize,
}
*/