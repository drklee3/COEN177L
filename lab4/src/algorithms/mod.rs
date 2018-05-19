#[cfg(feature = "fifo")]
pub mod fifo;
#[cfg(feature = "lru")]
pub mod lru;
#[cfg(feature = "second_chance")]
pub mod second_chance;

/*
use std;

pub trait ReplacementAlgorithm<T> {
  fn new() -> Self;

  fn get_page_table(&self) -> Vec<T>;

  fn is_in_memory(&self, page_request: T)
    -> bool where T: std::cmp::PartialEq {
    self.get_page_table().contains(&page_request)
  }
}
*/