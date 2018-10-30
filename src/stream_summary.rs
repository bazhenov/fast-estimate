use std::rc::{Rc, Weak};
use std::collections::{HashMap, LinkedList};

/// Stream Summary structure
pub struct StreamSummary {
  monitored_items: HashMap<String, Rc<Item>>,
  buckets: HashMap<u32, Bucket>
}

// Bucket is a set of elements sharing the same frequency of occurency in a stream
pub struct Bucket {
  head: Weak<Item>
}

impl Bucket {

  fn new() -> Bucket {
    Bucket { head: Weak::new() }
  }
}

pub struct Item {
  data: String,
  epsilon: u32,
  count: u32,
  next: Weak<Item>,
  prev: Weak<Item>,
  bucket: Weak<Bucket>
}

fn get_bucket(buckets: &mut HashMap<u32, Bucket>, order: u32) -> &mut Bucket {
  buckets.entry(order).or_insert_with(|| {
    Bucket::new()
  })
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

  pub fn offer(&mut self, data: &str) -> u32 {
    let item = self.monitored_items.entry(data.to_string()).or_insert_with(|| {
      Rc::new(Item { data: data.to_string(), epsilon: 0, count: 0,
        next: Weak::new(), prev: Weak::new(), bucket: Weak::new() })
    });

    let count;
    {
      /*let bucket = item.bucket.upgrade().unwrap();
      if Rc::ptr_eq(&bucket.head.upgrade().unwrap(), item) {
        match item.bucket.upgrade().as_mut() {
          Some(r) => Rc::get_mut(r).unwrap().head = Weak::new(),
          None => {}
        };
      }*/

      println!("RC: strong: {}, weak: {}", Rc::strong_count(item), Rc::weak_count(item));
      let i = Rc::get_mut(item).unwrap();
      i.count += 1;
      count = i.count;
    }
    //let l = self.buckets.push_back(Bucket{});
    let bucket = get_bucket(&mut self.buckets, count);
    bucket.head = Rc::downgrade(&Rc::clone(&item));
    item.bucket = Weak::new();
    count
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
