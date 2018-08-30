extern crate md5;

pub struct LinearCounter {
  buffer: Vec<u8>
}

impl LinearCounter {

  pub fn new(size: usize) -> LinearCounter {
    if size > 1000000 {
      panic!("Too much of buffer")
    }
    let mut buffer = Vec::with_capacity(size);
    for i in 0..size {
      buffer.push(0);
    }
    LinearCounter { buffer: buffer }
  }

  pub fn offer(&mut self, s: &str) {
    let hash = md5::compute(s);
    self.buffer[0] |= 1;
  }

  pub fn estimate(&self) -> u32 {
    0
  }

  fn population_count(&self) -> u32 {
    let mut r: u32 = 0;
    for i in 0..self.buffer.len() {
      let byte = self.buffer[i];
      for j in 0..8 {
        r += (byte & (1 << j)) as u32;
      }
    }
    r
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn it_works() {
    let mut lc = LinearCounter::new(10);
    lc.offer("Hello");
    assert_eq!(lc.population_count(), 1);
    //assert_eq!(lc.estimate(), 1)
  }
}
