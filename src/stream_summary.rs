use std::collections::{HashMap};
use double_linked_list::{DoublyLinkedList, NodeLink};

/// Stream Summary structure
#[allow(dead_code)]
pub struct StreamSummary {
  monitored_items: HashMap<String, Item>,
  buckets: HashMap<u32, DoublyLinkedList<String>>
}

#[allow(dead_code)]
pub struct Item {
  data: String,
  bucket_node: NodeLink<String>,
  epsilon: u32,
  count: u32
}

impl Item {

  fn new(data: &str, node: NodeLink<String>) -> Self {
    Item {
      data: data.to_string(),
      epsilon: 0,
      bucket_node: node,
      count: 1
    }
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

  fn push_item_to_bucket(buckets: &mut HashMap<u32, DoublyLinkedList<String>>, bucket: u32, data: &String) -> NodeLink<String> {
    buckets.entry(bucket)
      .or_insert_with(DoublyLinkedList::new)
      .push_back(data)
  }

  pub fn offer(&mut self, data: &str) -> u32 {
    let new_count = if self.monitored_items.contains_key(data) {
      let item = self.monitored_items.get_mut(data).unwrap();
      let count = item.count;
      let next_count = count + 1;
      item.count = next_count;

      // Removing item from current bucket
      let last_element_in_bucket = {
        let bucket = self.buckets.get_mut(&count).expect("Illegal state");
        bucket.remove(&item.bucket_node);
        bucket.empty()
      };
      if last_element_in_bucket {
        self.buckets.remove(&count);
      }

      // Adding item to the next bucket
      item.bucket_node = Self::push_item_to_bucket(&mut self.buckets, next_count, &item.data);

      next_count

    } else {
      // Pusing
      let node = Self::push_item_to_bucket(&mut self.buckets, 1, &data.to_string());
      let item = Item::new(data, node);
      self.monitored_items.insert(data.to_string(), item);

      1
    };

    new_count
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
    let mut s = StreamSummary::new();
    for i in 0..10 {
      assert_eq!(i + 1, s.offer("Hello"));
    }
  }
}
