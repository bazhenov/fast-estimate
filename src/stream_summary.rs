use std::rc::{Rc, Weak};
use std::collections::{HashMap};
use std::cell::RefCell;

/// Stream Summary structure
#[allow(dead_code)]
pub struct StreamSummary {
  monitored_items: HashMap<String, Rc<Item>>,
  buckets: HashMap<u32, Bucket>
}

// Bucket is a set of elements sharing the same frequency of occurency in a stream
#[allow(dead_code)]
pub struct Bucket {
  head: Weak<Item>
}

impl Bucket {

  fn new() -> Bucket {
    Bucket { head: Weak::new() }
  }
}

#[allow(dead_code)]
pub struct Item {
  data: String,
  epsilon: u32,
  count: u32,
  next: Option<Rc<RefCell<Item>>>,
  prev: Weak<Item>,
  bucket: Weak<Bucket>
}

impl Item {

  pub fn new(data: &str) -> Item {
    let item = Item {
      data: data.to_string(),
      epsilon: 0,
      count: 0,
      next: None,
      prev: Weak::new(),
      bucket: Weak::new()
    };
    //let r = Rc::new(RefCell::new(item));
    //r.borrow_mut().next = Some(Rc::clone(&r));
    item
  }

}

#[allow(dead_code)]
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

  pub fn offer(&mut self, _data: &str) -> u32 {

    // let item = self.monitored_items.entry(data.to_string()).or_insert_with(|| {
    //   Rc::new(Item { data: data.to_string(), epsilon: 0, count: 0,
    //     next: Rc::new(Item::new("Hi")), prev: Weak::new(), bucket: Weak::new() })
    // });
    //
    // let new_count = item.count + 1;
    // let bucket = get_bucket(&mut self.buckets, new_count);
    //
    // let count;
    // {
    //   let mut bucket = match item.bucket.upgrade() {
    //     Some(r) => r,
    //     None => panic!("Empty bucket reference on Item found")
    //   };
    //   match bucket.head.upgrade() {
    //     Some(bucket_head) => {
    //       if Rc::ptr_eq(&bucket_head, item) {
    //         match Rc::get_mut(&mut bucket) {
    //           Some(b) => b.head = Weak::new(),
    //           None => panic!("Bucket can't be borrowed mutably")
    //         }
    //       }
    //     },
    //     None => panic!("Empty item reference on Bucket found")
    //   }
    //   if Rc::ptr_eq(&bucket.head.upgrade().unwrap(), item) {
    //     match item.bucket.upgrade().as_mut() {
    //       Some(r) => Rc::get_mut(r).unwrap().head = Weak::new(),
    //       None => {}
    //     };
    //   }
    //
    //   println!("RC: strong: {}, weak: {}", Rc::strong_count(item), Rc::weak_count(item));
    //   let i = Rc::get_mut(item).unwrap();
    //   i.count += 1;
    //   count = i.count;
    // }
    //
    // bucket.head = Rc::downgrade(&Rc::clone(&item));
    // Rc::get_mut(item).unwrap().bucket = Weak::new();
    // count
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

  #[test]
  fn items_are_organized_in_ring() {
    let a: Item = Item::new("foo");
    let b: Item = Item::new("bar");
  }

  #[test]
  fn item_can_advance_on_item() {
    let _item = Item::new("first");

  }
}
