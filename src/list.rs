use std::rc::Rc;
use std::cell::RefCell;

pub struct List<T> {

  head: Link<T>,
  tail: Link<T>
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
  next: Link<T>,
  prev: Link<T>
}

impl<T> Node<T> {

  pub fn new_with_next(_value: T, next: &Rc<RefCell<Node<T>>>) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      next: Some(Rc::clone(next)), prev: None
    }))
  }

  pub fn new(_value: T) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      next: None, prev: None
    }))
  }
}

impl<T> List<T> {

  pub fn new() -> Self {
    Self {
      head: None,
      tail: None
    }
  }

  pub fn push_front(&mut self, value: T) {
    match self.head.take() {
      Some(head) => {
        self.head = Some(Node::new_with_next(value, &head));
      }
      None => {
        let node = Node::new(value);
        self.head = Some(node.clone());
        self.tail = Some(node);
      }
    }
  }

  pub fn size(&self) -> u32 {
    return match &self.head {
      None => 0,
      Some(head) => {
        let mut count = 1;
        let mut i: Link<T> = Some(Rc::clone(head));
        while (*i.as_ref().unwrap().borrow()).next.is_some() {
          i = {
            count += 1;
            let rc = &i.as_ref().unwrap();
            let ref_node = rc.borrow();
            let ref n = (*ref_node).next;
            Some(Rc::clone(n.as_ref().unwrap()))
          };
        }
        count
      }
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_a() {
    let mut l:List<u8> = List::new();
    l.push_front(5);
    l.push_front(6);
    l.push_front(7);
    assert_eq!(l.size(), 3);
  }

  #[test]
  fn test_refcell() {
    let rc: RefCell<u8> = RefCell::new(5);
    rc.replace(8);
    assert_eq!(8, *rc.borrow());
  }
}
