use std::collections::{HashMap, LinkedList};

/// Stream Summary structure
pub struct StreamSummary {
  monitored_items: HashMap<String, Item>,
  buckets: LinkedList<Bucket>
}

// Bucket is a set of elements sharing the same frequency of occurency in a stream
pub struct Bucket {
  items: LinkedList<Item>
}

pub struct Item {
  data: String,
  epsilon: u32,
}

impl StreamSummary {

  pub fn new() -> StreamSummary {
    return StreamSummary {
      monitored_items: HashMap::new(),
      buckets: LinkedList::new()
    };
  }

  pub fn estimate_top(&self) -> Vec<Item> {
    vec![]
  }

  pub fn offer(&mut self, data: &str) -> bool {
    self.monitored_items.entry(data.to_string()).or_insert_with(|| {
      return Item { data: data.to_string(), epsilon: 0 };
    });
    true
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
  fn should_find_already_offered_items() {
    let mut s = StreamSummary::new();
    assert_eq!(true, s.offer("Hello"));
    assert_eq!(false, s.offer("Hello"));
  }
}
