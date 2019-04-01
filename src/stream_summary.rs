use std::collections::{HashMap, BTreeMap};
use double_linked_list::{DoublyLinkedList, NodeLink};

type BucketMap = BTreeMap<usize, DoublyLinkedList<String>>;

/// Stream Summary structure.
///
/// Stream summary algorithm originally described in paper:
/// <a href="http://www.cse.ust.hk/~raywong/comp5331/References/EfficientComputationOfFrequentAndTop-kElementsInDataStreams.pdf">
/// Efficient Computation of Frequent and Top-k Elements in Data Streams</a>. It allows
/// estimate top-k elements in a stream storing only constant number of elements in a memory.
///
/// # Examples
/// ```
/// let mut stream  = StreamSummary::new();
///
/// stream.offer("hello");
/// stream.offer("hello");
/// stream.offer("world");
///
/// let top = stream.estimate_top();
///
/// assert_eq("hello", top[0].data);
/// assert_eq(2, top[0].count);
/// ```
pub struct StreamSummary {
  monitored_items: HashMap<String, Item>,
  buckets: BucketMap,
  capacity: usize
}

pub struct Item {
  pub data: String,
  bucket_node: NodeLink<String>,
  pub epsilon: usize,
  pub count: usize
}

impl Clone for Item {

  fn clone(&self) -> Self {
    Item {
      data: self.data.clone(),
      bucket_node: self.bucket_node.clone(),
      epsilon: self.epsilon,
      count: self.count
    }
  }
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

  pub fn new() -> Self {
    Self::with_capacity(1000)
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      monitored_items: HashMap::with_capacity(capacity),
      buckets: BTreeMap::new(),
      capacity: capacity
    }
  }

  pub fn estimate_top(&self) -> Vec<&Item> {
    let mut top : Vec<&Item> = self.monitored_items.values().collect();

    top.sort_unstable_by(|a, b| b.count.cmp(&a.count));
    top
  }

  fn push_item_to_bucket(buckets: &mut BucketMap, bucket: usize, data: &String) -> NodeLink<String> {
    buckets.entry(bucket)
      .or_insert_with(DoublyLinkedList::new)
      .push_back(data)
  }

  pub fn offer(&mut self, data: &str) -> usize {
    if self.monitored_items.contains_key(data) {
      // Incrementing count on exisiting element
      let item = self.monitored_items.get_mut(data).unwrap();
      let count = item.count;
      let next_count = count + 1;
      item.count = next_count;

      // Removing item from current bucket
      let should_remove_bucket = {
        let bucket = self.buckets.get_mut(&count).expect("Illegal state");
        bucket.remove(&item.bucket_node);
        bucket.empty()
      };
      if should_remove_bucket {
        self.buckets.remove(&count);
      }

      // Adding item to the next bucket
      item.bucket_node = Self::push_item_to_bucket(&mut self.buckets, next_count, &item.data);

      return next_count;

    } else {
      if self.monitored_items.len() >= self.capacity {
        // Replacing exisiting element
        let min_bucket = *self.buckets.keys().min().expect("No element in visited items found");

        let (item, should_remove_bucket) = {
          let bucket = self.buckets.get_mut(&min_bucket).expect("No bucket found!");
          let node = bucket.pop_front().expect("No element in a bucket found!");
          (self.monitored_items.remove(&node).unwrap(), bucket.empty())
        };

        if should_remove_bucket {
          self.buckets.remove(&min_bucket);
        }

        let new_count = item.count + 1;
        let new_epsilon = item.epsilon + 1;
        let new_node = Self::push_item_to_bucket(&mut self.buckets, new_count, &data.to_string());
        let item = Item {data: data.to_string(), bucket_node: new_node, epsilon: new_epsilon, count: new_count};
        self.monitored_items.insert(data.to_string(), item);

        return 1;
      } else {
        // Pushing new element
        let node = Self::push_item_to_bucket(&mut self.buckets, 1, &data.to_string());
        let item = Item::new(data, node);
        self.monitored_items.insert(data.to_string(), item);

        return 1;
      }
    };
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
  fn should_export_top_1() {
    let mut s = StreamSummary::new();
    s.offer("Hello");
    let top = s.estimate_top();
    assert_eq!(1, top.len());
    assert_eq!("Hello", top[0].data);
  }

  #[test]
  fn should_export_top_2() {
    let mut s = StreamSummary::new();
    s.offer("Hello");

    s.offer("world");
    s.offer("world");

    assert_eq!(vec!["world", "Hello"], top_items(&s));
  }

  #[test]
  fn export_less_than_visited() {
    let mut s = StreamSummary::with_capacity(2);

    offer(4, &mut s, "foo");
    offer(2, &mut s, "bar");
    offer(1, &mut s, "baz");

    assert_eq!(vec!("foo", "baz"), top_items(&s));
  }

  #[test]
  fn replace_values() {
    let mut s = StreamSummary::with_capacity(2);

    for i in 1..100 {
      s.offer(&i.to_string());
    }
  }

  fn offer(n: usize, s: &mut StreamSummary, data: &str) {
    for _ in 0..n {
      s.offer(data);
    }
  }

  fn top_items(summary: &StreamSummary) -> Vec<&String> {
    summary.estimate_top().iter()
      .map(|i| &i.data)
      .collect()
  }

  #[test]
  fn should_count_occurrences_correctly() {
    let mut s = StreamSummary::new();
    for i in 0..10 {
      assert_eq!(i + 1, s.offer("Hello"));
    }
  }
}
