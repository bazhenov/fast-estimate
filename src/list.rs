use std::rc::Rc;
use std::cell::RefCell;

pub struct List<T> {
  head: Link<T>,
  tail: Link<T>
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
	value: T,
  next: Link<T>,
  prev: Link<T>
}

impl<T: Copy> Node<T> {

  pub fn new_with_next(value: T, next: &Rc<RefCell<Node<T>>>) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
			value: value,
      next: Some(Rc::clone(next)),
			prev: None
    }))
  }

  pub fn new(value: T) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
			value: value,
      next: None,
			prev: None
    }))
  }
}

impl<T: Copy> List<T> {

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
        self.head = Some(Rc::clone(&node));
        self.tail = Some(node);
      }
    }
  }

	pub fn pop_front(&mut self) -> Option<T> {
		match self.head.take() {
			None => None,
			Some(head) => {
				let mut item = head.borrow_mut();
				self.head = match item.next.take() {
					None => None,
					Some(next) => {
						next.borrow_mut().prev = None;
						Some(Rc::clone(&next))
					}
				};
				Some(item.value)
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
          let i_next = match i.borrow().next {
            Some(ref nx) => {
              count += 1;
              Rc::clone(nx)
            }
            None => { break; }
          };
          i = i_next;
        }
        count
      }
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;
	use std::ptr;

  #[test]
  fn list_can_report_size() {
    let mut l:List<u8> = List::new();
    for _ in 0..5 {
      l.push_front(5);
    }
    assert_eq!(l.size(), 5);
  }

	#[test]
	fn list_can_pop() {
		let mut l:List<u8> = List::new();
		l.push_front(5);
		let v = l.pop_front();
		assert_eq!(v, Some(5));
	}

  #[derive(Debug)]
  struct Nd {
    n: Option<Rc<RefCell<Nd>>>,
    p: Option<Rc<RefCell<Nd>>>,
    v: u8
  }

  #[test]
  fn test_pointers_equality() {
    let a = Nd { n: None, p: None, v:1 };
    let b = Nd { n: None, p: None, v:2 };
    let c = Nd { n: None, p: None, v:3 };

    let a_rc = Rc::new(RefCell::new(a));
    let b_rc = Rc::new(RefCell::new(b));
    let c_rc = Rc::new(RefCell::new(c));

    a_rc.borrow_mut().n = Some(Rc::clone(&c_rc));
    c_rc.borrow_mut().p = Some(Rc::clone(&a_rc));

    print_list(&a_rc);

    let before: Rc<RefCell<Nd>> = Rc::clone(&a_rc);
    let after: Rc<RefCell<Nd>> = Rc::clone(&a_rc.borrow().n.as_ref().expect("Ooops"));

    before.borrow_mut().n = Some(Rc::clone(&b_rc));
    after.borrow_mut().p = Some(Rc::clone(&b_rc));
    b_rc.borrow_mut().n = Some(Rc::clone(&c_rc));
    b_rc.borrow_mut().p = Some(Rc::clone(&c_rc));

    print_list(&a_rc);
  }

  fn print_list(head: &Rc<RefCell<Nd>>) {
    let mut i = Some(Rc::clone(&head));
    loop {
      let i_temp = match i {
        Some(ref nd) => {
          print!("{:} -> ", nd.borrow().v);
          nd.borrow().n.clone()
        },
        None => {
          break
        }
      };
      i = i_temp;
    }
    println!()
  }

	//#[test]
	fn list_addresses() {
		let mut l = List::new();
		l.push_front(5);
		let head: &Rc<RefCell<Node<u8>>> = l.head.as_ref().unwrap();
		let tail: &Rc<RefCell<Node<u8>>> = l.tail.as_ref().unwrap();
		assert_eq!(ptr::eq(head, tail), true);
		//println!("Head is: {:p}", l.head.unwrap());
		//println !("Tail is: {:p}", l.tail.unwrap());
	}
}
