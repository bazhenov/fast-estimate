#[macro_use]
extern crate clap;
mod LinearCounter;

use clap::{Arg, App};
use std::io;

#[derive(Debug)]
struct LinearCounterConfig {
  buffer_size: usize
}

fn main() {
  let matches = App::new("linear counter")
    .version("1.0")
    .about("Estimating using linear counting")
    .arg(Arg::with_name("size")
      .short("s")
      .long("sz")
      .help("Set the size of buffer")
      .takes_value(true))
    .get_matches();

  let buffer_size = value_t!(matches.value_of("size"), usize).unwrap_or_else(|e| e.exit());

  let config = LinearCounterConfig { buffer_size: buffer_size };
  let lc = LinearCounter::LinearCounter::new(config.buffer_size);
  println!("{:?}", config);

  let stdin = io::stdin();
  let mut line = String::new();
  loop {
    line.clear();
    match stdin.read_line(&mut line) {
      Ok(n) => {
        if n <= 0 {
          break;
        }
        println!("Line is: {:}", line.trim());
      }
      Err(e) => {
        panic!("{:?}", e)
      }
    }
  }

  //println!("{:2?}", lc.buffer)
}
