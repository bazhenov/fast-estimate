use std::cell::{Ref, RefMut};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct NodeLink(Rc<RefCell<Node>>);

type Link = Option<NodeLink>;

pub struct DoublyLinkedList {

  head: Link,
  tail: Link
}

struct Node {
  data: String,
  next: Link,
  prev: Weak<RefCell<Node>>
}

impl NodeLink {

  fn new(node: Node) -> NodeLink {
    NodeLink(Rc::new(RefCell::new(node)))
  }

  fn borrow(&self) -> Ref<Node> {
    self.0.borrow()
  }

  fn borrow_mut(&self) -> RefMut<Node> {
    self.0.borrow_mut()
  }

  fn weak(&self) -> Weak<RefCell<Node>> {
    Rc::downgrade(&self.0)
  }

  fn upgrade_prev(&self) -> Option<NodeLink> {
    self.0.borrow().prev.upgrade().map(NodeLink)
  }
}

impl Clone for NodeLink {

  fn clone(&self) -> NodeLink {
    NodeLink(Rc::clone(&self.0))
  }
}

impl Node {

  fn new_standalone(value: &str) -> NodeLink {
    let node = Node { data: value.to_string(), next: None, prev: Weak::new() };
    NodeLink::new(node)
  }

  fn new_tail(value: &str, old_tail: &Link) -> NodeLink {
    let node = Node { data: value.to_string(), next: None, prev: weak_link(old_tail) };
    NodeLink::new(node)
  }

  fn new_head(value: &str, old_head: &Link) -> NodeLink {
    let node = Node { data: value.to_string(), next: clone_link(old_head), prev: Weak::new() };
    NodeLink::new(node)
  }
}

fn clone_link(link: &Link) -> Link {
  link.as_ref().map(NodeLink::clone)
}

fn weak_link(link: &Link) -> Weak<RefCell<Node>> {
  link.as_ref().map(NodeLink::weak).unwrap()
}

impl DoublyLinkedList {

  fn new() -> DoublyLinkedList {
    DoublyLinkedList { head: None, tail: None }
  }

  fn iter(&self) -> DoublyLinkedListIterator {
    DoublyLinkedListIterator { item: self.head.as_ref().map(NodeLink::clone) }
  }

  fn push_back(&mut self, value: &str) -> NodeLink {
    let new_tail = if let Some(ref old_tail) = self.tail {
      let new_tail = Node::new_tail(value, &self.tail);
      old_tail.borrow_mut().next = Some(new_tail.clone());
      new_tail
    } else {
      let head = Node::new_standalone(value);
      self.head = Some(head.clone());
      head.clone()
    };
    self.tail = Some(new_tail.clone());
    new_tail
  }

  fn push_front(&mut self, value: &str) -> NodeLink {
    let new_head : NodeLink = if let Some(ref old_head) = self.head  {
      let new_head = Node::new_head(value, &self.head);
      old_head.borrow_mut().prev = new_head.weak();
      new_head
    } else {
      let head = Node::new_standalone(value);
      // updating tail beacause it's the first element in the list
      self.tail = Some(head.clone());
      head
    };
    self.head = Some(new_head.clone());
    new_head
  }

  fn pop_front(&mut self) -> Option<String> {
    let (value, new_head) = if let Some(ref old_head) = self.head {
      (Some(old_head.borrow().data.clone()), clone_link(&old_head.borrow().next))
    } else {
      (None, None)
    };
    self.head = new_head;
    if self.head.is_none() {
      self.tail = None;
    }
    value
  }

  fn pop_back(&mut self) -> Option<String> {
    let (value, new_tail) = if let Some(ref old_tail) = self.tail {
      (Some(old_tail.borrow().data.clone()), old_tail.upgrade_prev())
    } else {
      (None, None)
    };
    self.tail = new_tail;
    if self.tail.is_none() {
      self.head = None;
    }
    value
  }

  /// Returns length of a list
  ///
  /// ```rust
  /// let mut list = DoublyLinkedList::new();
  /// list.push_back("Hello");
  /// assert_eq!(2, list.len());
  /// ```
  pub fn len(&self) -> usize {
    self.iter().count()
  }
}

struct DoublyLinkedListIterator {
  item: Option<NodeLink>
}

impl Iterator for DoublyLinkedListIterator {

  type Item = String;

  fn next(&mut self) -> Option<Self::Item> {
    let(ret, next) = match self.item {
      Some(ref i) => {
        let r: Ref<Node> = i.borrow();
        (Some(r.data.clone()), r.next.as_ref().map(NodeLink::clone))
      },
      None => (None, None)
    };
    self.item = next;
    ret
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn empty_size() {
    let mut list = DoublyLinkedList::new();
    assert_empty(&mut list);
  }

  #[test]
  fn one_element_push_back_size() {
    let mut list = DoublyLinkedList::new();
    list.push_back("Hello");
    assert_eq!(1, list.len());
  }

  #[test]
  fn two_element_push_back_size() {
    let mut list = DoublyLinkedList::new();
    list.push_back("Hello");
    list.push_back("World");
    assert_eq!(2, list.len());
  }

  #[test]
  fn two_element_push_front_size() {
    let mut list = DoublyLinkedList::new();
    list.push_front("World");
    list.push_front("Hello");
    assert_eq!(2, list.len());
  }

  #[test]
  fn two_element_push_back_collect() {
    let mut list = DoublyLinkedList::new();
    list.push_back("Hello");
    list.push_back("world");
    let vector: Vec<String> = list.iter().collect();
    assert_eq!("Hello, world", vector.join(", "));
  }

  #[test]
  fn two_element_push_front_collect() {
    let mut list = DoublyLinkedList::new();
    list.push_front("world");
    list.push_front("Hello");
    let vector: Vec<String> = list.iter().collect();
    assert_eq!("Hello, world", vector.join(", "));
  }

  #[test]
  fn empty_pop_front() {
    let mut list = DoublyLinkedList::new();
    assert_eq!(None, list.pop_front());
  }

  #[test]
  fn one_element_pop_front() {
    let mut list = DoublyLinkedList::new();
    list.push_back("hello");
    assert_eq!("hello", list.pop_front().unwrap());
    assert_empty(&mut list);
  }

  #[test]
  fn two_element_pop_front() {
    let mut list = DoublyLinkedList::new();
    list.push_back("hello");
    list.push_back("world");
    assert_eq!("hello", list.pop_front().unwrap());
    assert_eq!("world", list.pop_front().unwrap());
    assert_empty(&mut list);
  }

  #[test]
  fn one_element_pop_back() {
    let mut list = DoublyLinkedList::new();
    list.push_back("hello");
    assert_eq!("hello", list.pop_back().unwrap());
    assert_empty(&mut list);
  }

  #[test]
  fn can_store_node() {
    let mut list = DoublyLinkedList::new();
    let node = list.push_front("hello");
    assert_eq!("hello", node.borrow().data);

    let node = list.push_back("world");
    assert_eq!("world", node.borrow().data);
  }

  #[test]
  fn links_should_be_updated() {
    let mut list = DoublyLinkedList::new();
    let head = list.push_front("hello");

    assert!(head.borrow().next.is_none());
    let tail = list.push_back("world");

    assert!(Rc::ptr_eq(&tail.0, &head.borrow().next.as_ref().unwrap().0));
  }

  #[test]
  fn check_rc_links_count() {
    let mut list = DoublyLinkedList::new();
    let node = list.push_back("first");

    // We shold have 3 references at this point: head, tail and returned one (node)
    assert_eq!(3, Rc::strong_count(&node.0));

    // We still should have 3 refs: first element, tail of the list and "node"-binding
    list.push_front("second");
    assert_eq!(3, Rc::strong_count(&node.0));

    // At this point tail reference is moved to another element. So we're expecting
    // 2 refs here: first element referencing second and "node" binding itself.
    list.push_back("third");
    assert_eq!(2, Rc::strong_count(&node.0));
  }

  fn assert_empty(list: &mut DoublyLinkedList) {
    assert_eq!(0, list.len());
    assert_eq!(None, list.pop_front());
    assert_eq!(None, list.pop_back());
  }
}
