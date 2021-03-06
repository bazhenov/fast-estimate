extern crate md5;

/// Linear counting structure
///
/// Basically linear counter is the bit array. Each incoming item is associated with single bit
/// using hash function.
///
/// Detailed explanation of the algorithm: [A Linear-Time Probabilistic Counting Algorithm
/// for Database Applications](http://dblab.kaist.ac.kr/Prof/pdf/ACM90_TODS_v15n2.pdf)
pub struct LinearCounter {
  buffer: Vec<u32>
}

const MAX_BUFFER_SIZE: usize = (u32::max_value() / 8 / 4) as usize;

impl LinearCounter {

  pub fn new(size: usize) -> LinearCounter {
    if size > MAX_BUFFER_SIZE {
      panic!("Too much of buffer")
    }
    LinearCounter { buffer: vec![0; size] }
  }

  /// Updates a counter with given string
  pub fn offer(&mut self, s: &str) {
    let digest = md5::compute(s);
    let bit_idx = self.calculate_bit_idx(&digest);

    // Use first 5 bits as bit offset and the rest as vector word (u32) offset
    let bit_offset = bit_idx & 0b11111;
    let byte_offset = bit_idx >> 5;
    self.buffer[byte_offset] |= 1 << bit_offset;
  }

  /// Estimates a number of unique elemnts given to the `offer` method
  pub fn estimate(&self) -> u32 {
    let l: f64 = self.buffer.len() as f64;
    let nf: f64 = l - self.population_count() as f64;
    return (l * (l / nf).ln()).round() as u32;
  }

  /// Calculate bit index in the buffer linked to given hash sum
  fn calculate_bit_idx(&self, digest: &md5::Digest) -> usize {
    let mut num = digest[0] as usize;
    num <<= 8;
    num |= digest[1] as usize;
    num <<= 8;
    num |= digest[2] as usize;
    num <<= 8;
    num |= digest[3] as usize;
    return num % (self.buffer.len() * 32);
  }

  fn population_count(&self) -> u32 {
    let mut r: u32 = 0;
    let mut i: usize = self.buffer.len();
    loop {
      i -= 1;
      r += pop_count(self.buffer[i]);
      if i <= 0 {
        break;
      }
    }
    return r
  }
}

#[inline]
fn pop_count(i: u32) -> u32 {
  let mut i: u32 = i - ((i >> 1) & 0x55555555);
  i = (i & 0x33333333) + ((i >> 2) & 0x33333333);
  return (((i + (i >> 4)) & 0x0F0F0F0F).wrapping_mul(0x01010101)) >> 24;
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
      let mut lc = LinearCounter::new(1000000);
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
    let lc = LinearCounter::new(1000);
    assert_eq!(lc.population_count(), 0)
  }


  #[test]
  fn new_pop_count() {
    assert_eq!(pop_count(0x00000000), 0);
    assert_eq!(pop_count(0xFFFFFFFF), 32);
    assert_eq!(pop_count(0xFF0F0F00), 16);
  }
}
