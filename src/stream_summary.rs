use std::rc::{Rc, Weak};
use std::collections::{HashMap};
use std::cell::RefCell;
use double_linked_list::{DoublyLinkedList, NodeLink};

/// Stream Summary structure
#[allow(dead_code)]
pub struct StreamSummary {
  monitored_items: HashMap<String, Rc<Item>>,
  buckets: HashMap<u32, DoublyLinkedList<String>>
}

#[allow(dead_code)]
pub struct Item {
  data: String,
  //bucket_node: NodeLink<String>,
  epsilon: u32,
  count: u32
}

impl Item {

  pub fn new(data: &str) -> Item {
    let item = Item {
      data: data.to_string(),
      epsilon: 0,
      count: 0
    };
    item
  }

}

impl StreamSummary {

  pub fn new() -> StreamSummary {
    return StreamSummary {
      monitored_items: HashMap::new(),
      buckets: HashMap::new()
    };
  }

  pub fn estimate_top(&self) -> Vec<Item> {
    vec![]
  }

  pub fn offer(&mut self, _data: &str) -> u32 {
    0
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn shound() {
    let s = StreamSummary::new();
    assert_eq!(0, s.estimate_top().len());
  }

  #[test]
  fn should_count_occurrences_correctly() {
    //let mut s = StreamSummary::new();
    // for i in 0..10 {
    //   assert_eq!(i + 1, s.offer("Hello"));
    // }
  }
}
