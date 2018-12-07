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
        let mut i: Rc<RefCell<Node<T>>> = Rc::clone(head);
        loop {
          let i_temp = match i.borrow().next {
            Some(ref nx) => {
              count += 1;
              Rc::clone(nx)
            }
            None => { break; }
          };
          i = i_temp;
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
  fn list_can_report_size() {
    let mut l:List<u8> = List::new();
    for i in 0..5 {
      l.push_front(5);
    }
    assert_eq!(l.size(), 5);
  }
}
