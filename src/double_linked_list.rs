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

  fn new(node: Node) -> Self {
    NodeLink(Rc::new(RefCell::new(node)))
  }

  fn new_standalone(value: &str) -> Self {
    let node = Node { data: value.to_string(), next: None, prev: Weak::new() };
    Self::new(node)
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

  fn next(&self) -> Option<NodeLink> {
    self.borrow().next.as_ref().map(NodeLink::clone)
  }

  fn update_backward_link(&self, node: &NodeLink) {
    self.borrow_mut().prev = node.weak();
  }

  fn link_to(&self, successor: &NodeLink) {
    self.borrow_mut().next = Some(successor.clone());
    successor.borrow_mut().prev = self.weak();
  }

  fn create_before(&self, value: &str) -> Self {
    let node = Node { data: value.to_string(), next: Some(self.clone()), prev: Weak::new() };
    let new = Self::new(node);
    if let Some(ref prev) = self.upgrade_prev() {
      prev.link_to(&new);
    }
    new.link_to(self);
    new
  }

  fn create_after(&self, value: &str) -> Self {
    let node = Node { data: value.to_string(), next: None, prev: self.weak() };
    let new = Self::new(node);
    if let Some(ref next) = self.0.borrow().next {
      new.link_to(next)
    }
    self.link_to(&new);
    new
  }

  fn next_link(&self) -> Option<Self> {
    self.0.borrow().next.as_ref().map(Self::clone)
  }

  fn upgrade_prev(&self) -> Option<Self> {
    self.0.borrow().prev.upgrade().map(Self)
  }

  fn ptr_eq(&self, other: &Self) -> bool {
    Rc::ptr_eq(&self.0, &other.0)
  }
}

impl Clone for NodeLink {

  fn clone(&self) -> Self {
    Self(Rc::clone(&self.0))
  }
}

impl DoublyLinkedList {

  fn new() -> DoublyLinkedList {
    DoublyLinkedList { head: None, tail: None }
  }

  fn iter(&self) -> DoublyLinkedListIterator {
    DoublyLinkedListIterator { item: self.head.as_ref().map(NodeLink::clone) }
  }

  fn head(&self) -> Option<NodeLink> {
    self.head.clone()
  }

  fn tail(&self) -> Option<NodeLink> {
    self.tail.clone()
  }

  fn push_back(&mut self, value: &str) -> NodeLink {
    let new_tail = if let Some(ref mut old_tail) = self.tail {
      old_tail.create_after(value)
    } else {
      let head = NodeLink::new_standalone(value);
      self.head = Some(head.clone());
      head
    };
    self.tail = Some(new_tail.clone());
    new_tail
  }

  fn push_front(&mut self, value: &str) -> NodeLink {
    let new_head : NodeLink = if let Some(ref mut old_head) = self.head  {
      old_head.create_before(value)
    } else {
      let head = NodeLink::new_standalone(value);
      // updating tail beacause it's the first element in the list
      self.tail = Some(head.clone());
      head
    };
    self.head = Some(new_head.clone());
    new_head
  }

  fn push_after(&mut self, node: &NodeLink, value: &str) -> NodeLink {
    node.create_after(value)
  }

  fn push_before(&mut self, node: &NodeLink, value: &str) -> NodeLink {
    node.create_before(value)
  }

  fn pop_front(&mut self) -> Option<String> {
    let (value, new_head) = if let Some(ref old_head) = self.head {
      (Some(old_head.borrow().data.clone()), old_head.next_link())
    } else {
      (None, None)
    };
    self.head = new_head;

    if let Some(ref head) = self.head {
      head.borrow_mut().prev = Weak::new();
    } else {
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

    if let Some(ref tail) = self.tail {
      tail.borrow_mut().next = None;
    } else {
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
        let r = i.borrow();
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
  fn one_element_get_head_and_tail() {
    let mut list = DoublyLinkedList::new();
    let node = list.push_back("hello");
    assert!(node.ptr_eq(list.head().as_ref().unwrap()));
    assert!(node.ptr_eq(list.tail().as_ref().unwrap()));
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
    assert_eq!("Hello, world", list_as_string(&list, ", "));
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
  fn push_after() {
    let mut list = DoublyLinkedList::new();
    list.push_back("world");
    let head = list.push_front("Brave");
    list.push_after(&head, "new");

    assert_eq!("Brave new world", list_as_string(&list, " "));
  }

  #[test]
  fn push_before() {
    let mut list = DoublyLinkedList::new();
    let tail = list.push_back("world");
    list.push_front("Brave");
    list.push_before(&tail, "new");

    assert_eq!("Brave new world", list_as_string(&list, " "));
  }

  // #[test]
  // fn push_before_updates_root() {
  //   let mut
  // }

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

  fn list_as_string(list: &DoublyLinkedList, separator: &str) -> String {
    list.iter().collect::<Vec<String>>().join(separator)
  }

  fn assert_empty(list: &mut DoublyLinkedList) {
    assert_eq!(0, list.len());
    assert_eq!(None, list.pop_front());
    assert_eq!(None, list.pop_back());
    assert!(list.head().is_none());
    assert!(list.tail().is_none());
  }
}
