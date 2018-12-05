#[macro_use]
extern crate clap;
pub mod linear_counter;
pub mod stream_summary;
pub mod list;

use clap::{Arg, App};
use std::io;

fn main() {
  let matches = App::new("linear counter")
    .version("1.0")
    .about("Estimating using linear counting")
    .arg(Arg::with_name("size")
      .short("s")
      .long("size")
      .help("Set the size of buffer (in 4 byte words)")
      .default_value("100000")
      .takes_value(true))
    .get_matches();

  let buffer_size = value_t!(matches, "size", usize).unwrap_or_else(|e| e.exit());

  let mut lc = linear_counter::LinearCounter::new(buffer_size);

  let stdin = io::stdin();
  let mut line = String::new();
  loop {
    line.clear();
    match stdin.read_line(&mut line) {
      Ok(n) => {
        if n <= 0 {
          break;
        }
        lc.offer(&line)
      }
      Err(e) => {
        panic!("{:?}", e)
      }
    }
  }

  println!("{:}", lc.estimate())
}
