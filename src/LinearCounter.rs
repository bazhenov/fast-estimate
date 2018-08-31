extern crate md5;

pub struct LinearCounter {
  buffer: Vec<u8>
}

const MAX_BUFFER_SIZE: usize = (u32::max_value() / 8) as usize;

impl LinearCounter {

  pub fn new(size: usize) -> LinearCounter {
    if size > MAX_BUFFER_SIZE {
      panic!("Too much of buffer")
    }
    let mut buffer = Vec::with_capacity(size);
    for i in 0..size {
      buffer.push(0);
    }
    LinearCounter { buffer: buffer }
  }

  pub fn offer(&mut self, s: &str) {
    let digest = md5::compute(s);
    let bit_idx = self.buffer_idx(&digest);
    let bit_offset = bit_idx & 0x07;
    let byte_offset = bit_idx >> 3;
    self.buffer[byte_offset] |= 1 << bit_offset;
  }

  pub fn estimate(&self) -> u32 {
    let l: f64 = self.buffer.len() as f64;
    let Nf: f64 = l - self.population_count() as f64;
    return (l * (l / Nf).ln()).round() as u32;
  }

  fn buffer_idx(&self, digest: &md5::Digest) -> usize {
    let mut num = digest[0] as usize;
    num <<= 8;
    num |= digest[1] as usize;
    num <<= 8;
    num |= digest[2] as usize;
    num <<= 8;
    num |= digest[3] as usize;
    return num % self.buffer.len();
  }

  fn population_count(&self) -> u32 {
    let mut r: u32 = 0;
    for i in 0..self.buffer.len() {
      let byte = self.buffer[i];
      for j in 0..8 {
        r += ((byte >> j) & 1) as u32;
      }
    }
    return r
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn population_count() {
    for i in 1..10 {
      let mut lc = LinearCounter::new(1000);
      for j in 0..i {
        let mut s = "str".to_string();
        s.push_str(&j.to_string());
        lc.offer(&s);
      }
      assert_eq!(lc.population_count(), i);
    }
  }

  #[test]
  fn estimate() {
    for i in 1..10 {
      let mut lc = LinearCounter::new(10000);
      for j in 0..i {
        let mut s = "str".to_string();
        s.push_str(&j.to_string());
        lc.offer(&s);
      }
      assert_eq!(lc.estimate(), i);
    }
  }

  #[test]
  fn zero_lc() {
    let mut lc = LinearCounter::new(1000);
    assert_eq!(lc.population_count(), 0)
  }
}
