use std::cell::{Ref, RefMut};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

type Link<T> = Option<NodeLink<T>>;

pub struct NodeLink<T>(Rc<RefCell<Node<T>>>);

/// Double linked list.
///
/// Usage:
/// ```
/// let mut list = DoublyLinkedList::new();
/// list.push_front("Hello");
/// list.push_front("World");
/// assert_eq!(2, list.len());
/// ```
///
/// You could also iterate over list:
/// ```
/// let mut list = DoublyLinkedList::new();
/// list.push_front("Hello");
/// list.push_front("World");
/// for item in list.iter() {
/// }
/// ```
pub struct DoublyLinkedList<T> {

  head: Link<T>,
  tail: Link<T>
}

pub struct DoublyLinkedListIterator<T> {
  item: Option<NodeLink<T>>
}

struct Node<T> {
  data: T,
  next: Link<T>,
  prev: Weak<RefCell<Self>>
}

impl<T: Clone> NodeLink<T> {

  fn new(node: Node<T>) -> Self {
    NodeLink(Rc::new(RefCell::new(node)))
  }

  fn new_standalone(value: &T) -> Self {
    let node : Node<T> = Node { data: value.clone(), next: None, prev: Weak::new() };
    Self::new(node)
  }

  fn borrow(&self) -> Ref<Node<T>> {
    self.0.borrow()
  }

  fn borrow_mut(&self) -> RefMut<Node<T>> {
    self.0.borrow_mut()
  }

  fn weak(&self) -> Weak<RefCell<Node<T>>> {
    Rc::downgrade(&self.0)
  }

  fn link_to(&self, successor: &NodeLink<T>) {
    self.borrow_mut().next = Some(successor.clone());
    successor.borrow_mut().prev = self.weak();
  }

  fn create_before(&self, value: &T) -> Self {
    let node = Node { data: value.clone(), next: Some(self.clone()), prev: Weak::new() };
    let new = Self::new(node);
    if let Some(ref prev) = self.upgrade_prev() {
      prev.link_to(&new);
    }
    new.link_to(self);
    new
  }

  fn create_after(&self, value: &T) -> Self {
    let node = Node { data: value.clone(), next: None, prev: self.weak() };
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

  fn ptr_eq_and_present(&self, other: &Option<Self>) -> bool {
    if let Some(ref node) = other {
      return Rc::ptr_eq(&self.0, &node.0);
    }
    false
  }
}

impl<T> Clone for NodeLink<T> {

  fn clone(&self) -> Self {
    Self(Rc::clone(&self.0))
  }
}

impl<T: Clone> DoublyLinkedList<T> {

  /// Creates new list
  pub fn new() -> Self {
    DoublyLinkedList { head: None, tail: None }
  }

  pub fn iter(&self) -> DoublyLinkedListIterator<T> {
    DoublyLinkedListIterator { item: self.head.as_ref().map(NodeLink::clone) }
  }

  /// Returns a head of the list. `None` if list has 0 elements.
  pub fn head(&self) -> Option<NodeLink<T>> {
    self.head.clone()
  }

  /// Returns a tail of the list. `None` if list has 0 elements.
  pub fn tail(&self) -> Option<NodeLink<T>> {
    self.tail.clone()
  }

  /// Adds element to an end of the list
  pub fn push_back(&mut self, value: &T) -> NodeLink<T> {
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

  pub  fn push_front(&mut self, value: &T) -> NodeLink<T> {
    let new_head = if let Some(ref mut old_head) = self.head  {
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

  pub fn push_after(&mut self, node: &NodeLink<T>, value: &T) -> NodeLink<T> {
    let new = node.create_after(value);

    // Updating tail if pushing after last element of a list
    if node.ptr_eq_and_present(&self.tail) {
      self.tail = Some(new.clone())
    }

    new
  }

  pub  fn push_before(&mut self, node: &NodeLink<T>, value: &T) -> NodeLink<T> {
    let new = node.create_before(value);

    // Updates head if pushing before first element in a list
    if node.ptr_eq_and_present(&self.head) {
      self.head = Some(new.clone());
    }

    new
  }

  pub  fn pop_front(&mut self) -> Option<T> {
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

  pub fn pop_back(&mut self) -> Option<T> {
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

  pub fn remove(&mut self, target: &NodeLink<T>) {
    match (target.ptr_eq_and_present(&self.head), target.ptr_eq_and_present(&self.tail)) {
      (true, true) => {
        self.head = None;
        self.tail = None;
      },
      (true, false) => {
        let new_head = target.next_link().unwrap();
        new_head.borrow_mut().prev = Weak::new();
        self.head = Some(new_head);
      },
      (false, true) => {
        let new_tail = target.upgrade_prev().unwrap();
        new_tail.borrow_mut().next = None;
        self.tail = Some(new_tail);
      },
      (false, false) => {
        let prev = target.upgrade_prev().unwrap();
        let next = target.next_link().unwrap();

        next.borrow_mut().prev = prev.weak();
        prev.borrow_mut().next = Some(next);
      }
    }
    target.borrow_mut().next = None;
    target.borrow_mut().prev = Weak::new();
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

impl<T: Clone> Iterator for DoublyLinkedListIterator<T> {

  type Item = T;

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
    let node = list.push_back(&"hello");
    assert!(node.ptr_eq_and_present(&list.head()));
    assert!(node.ptr_eq_and_present(&list.tail()));
  }

  #[test]
  fn push_after_updates_tail() {
    let mut list = DoublyLinkedList::new();
    let head = list.push_back(&"hello");
    let tail = list.push_after(&head, &"world");
    assert!(tail.ptr_eq_and_present(&list.tail()));
  }

  #[test]
  fn push_before_updates_head() {
    let mut list = DoublyLinkedList::new();
    let tail = list.push_back(&"hello");
    let head = list.push_before(&tail, &"world");
    assert!(head.ptr_eq_and_present(&list.head()));
  }

  #[test]
  fn one_element_push_back_size() {
    let mut list = DoublyLinkedList::new();
    list.push_back(&"Hello");
    assert_eq!(1, list.len());
  }

  #[test]
  fn two_element_push_back_size() {
    let mut list = DoublyLinkedList::new();
    list.push_back(&"Hello");
    list.push_back(&"World");
    assert_eq!(2, list.len());
  }

  #[test]
  fn two_element_push_front_size() {
    let mut list = DoublyLinkedList::new();
    list.push_front(&"World");
    list.push_front(&"Hello");
    assert_eq!(2, list.len());
  }

  #[test]
  fn two_element_push_back_collect() {
    let mut list = DoublyLinkedList::new();
    list.push_back(&"Hello");
    list.push_back(&"world");
    let vector: Vec<&str> = list.iter().collect();
    assert_eq!("Hello, world", vector.join(", "));
  }

  #[test]
  fn two_element_push_front_collect() {
    let mut list = DoublyLinkedList::new();
    list.push_front(&"world");
    list.push_front(&"Hello");
    assert_eq!("Hello, world", list_as_string(&list, ", "));
  }

  #[test]
  fn empty_pop_front() {
    let mut list : DoublyLinkedList<&str> = DoublyLinkedList::new();
    assert_eq!(None, list.pop_front());
  }

  #[test]
  fn one_element_pop_front() {
    let mut list = DoublyLinkedList::new();
    list.push_back(&"hello");
    assert_eq!("hello", list.pop_front().unwrap());
    assert_empty(&mut list);
  }

  #[test]
  fn two_element_pop_front() {
    let mut list = DoublyLinkedList::new();
    list.push_back(&"hello");
    list.push_back(&"world");
    assert_eq!("hello", list.pop_front().unwrap());
    assert_eq!("world", list.pop_front().unwrap());
    assert_empty(&mut list);
  }

  #[test]
  fn one_element_pop_back() {
    let mut list = DoublyLinkedList::new();
    list.push_back(&"hello");
    assert_eq!("hello", list.pop_back().unwrap());
    assert_empty(&mut list);
  }

  #[test]
  fn can_store_node() {
    let mut list = DoublyLinkedList::new();
    let node = list.push_front(&"hello");
    assert_eq!("hello", node.borrow().data);

    let node = list.push_back(&"world");
    assert_eq!("world", node.borrow().data);
  }

  #[test]
  fn links_should_be_updated() {
    let mut list = DoublyLinkedList::new();
    let head = list.push_front(&"hello");

    assert!(head.borrow().next.is_none());
    let tail = list.push_back(&"world");

    assert!(Rc::ptr_eq(&tail.0, &head.borrow().next.as_ref().unwrap().0));
  }

  #[test]
  fn push_after() {
    let mut list = DoublyLinkedList::new();
    list.push_back(&"world");
    let head = list.push_front(&"Brave");
    list.push_after(&head, &"new");

    assert_eq!("Brave new world", list_as_string(&list, " "));
  }

  #[test]
  fn push_before() {
    let mut list = DoublyLinkedList::new();
    let tail = list.push_back(&"world");
    list.push_front(&"Brave");
    list.push_before(&tail, &"new");

    assert_eq!("Brave new world", list_as_string(&list, " "));
  }

  #[test]
  fn remove_node() {
    let mut list = DoublyLinkedList::new();
    list.push_back(&"foo");
    let target = list.push_back(&"bar");
    list.push_back(&"baz");

    list.remove(&target);
    assert_eq!(2, list.len());
    assert_eq!("foo, baz", list_as_string(&list, ", "));

    assert_eq!(Some("foo"), list.pop_front());
    assert_eq!(Some("baz"), list.pop_back());
  }

  #[test]
  fn check_rc_links_count() {
    let mut list = DoublyLinkedList::new();
    let node = list.push_back(&"first");

    // We shold have 3 references at this point: head, tail and returned one (node)
    assert_eq!(3, Rc::strong_count(&node.0));

    // We still should have 3 refs: first element, tail of the list and "node"-binding
    list.push_front(&"second");
    assert_eq!(3, Rc::strong_count(&node.0));

    // At this point tail reference is moved to another element. So we're expecting
    // 2 refs here: first element referencing second and "node" binding itself.
    list.push_back(&"third");
    assert_eq!(2, Rc::strong_count(&node.0));
  }

  fn list_as_string(list: &DoublyLinkedList<&str>, separator: &str) -> String {
    list.iter()
      .map(str::to_string)
      .collect::<Vec<String>>()
      .join(separator)
      .to_string()
  }

  fn assert_empty(list: &mut DoublyLinkedList<&str>) {
    assert_eq!(0, list.len());
    assert_eq!(None, list.pop_front());
    assert_eq!(None, list.pop_back());
    assert!(list.head().is_none());
    assert!(list.tail().is_none());
  }
}
