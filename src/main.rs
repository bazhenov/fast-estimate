#[macro_use]
extern crate clap;
pub mod linear_counter;
pub mod stream_summary;
pub mod double_linked_list;

use clap::{Arg, App, SubCommand};
use std::io;

use stream_summary::StreamSummary;
use linear_counter::LinearCounter;

fn main() {

  let matches = build_cli().get_matches();

  let stdin = io::stdin();
  let mut stdout = io::stdout();
  let mut line = String::new();

  if let Some(matches) = matches.subcommand_matches("uniq") {
    let size = value_t!(matches, "size", usize).unwrap_or_else(|e| e.exit());
    let mut lc = LinearCounter::new(size);

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

  } else if let Some(matches) = matches.subcommand_matches("top") {
    let size = value_t!(matches, "size", usize).unwrap_or_else(|e| e.exit());
    let mut summary = StreamSummary::with_capacity(size);

    loop {
      line.clear();
      match stdin.read_line(&mut line) {
        Ok(n) => {
          if n <= 0 {
            break;
          }
          summary.offer(&line);

        }
        Err(e) => {
          panic!("{:?}", e)
        }
      }
    }

    for item in summary.estimate_top() {
      println!("{} - {}", item.count, item.data)
    }


  } else {
    build_cli().write_help(&mut stdout);
  }


}

fn build_cli() -> App<'static, 'static> {
  let top = SubCommand::with_name("top")
    .about("estimates a top-k values in a stream")
    .arg(Arg::with_name("size")
      .long("size")
      .short("s")
      .takes_value(true)
      .default_value("1000")
      .help("capacity of a stream-summary"));

  let uniq = SubCommand::with_name("uniq")
    .about("estimate number of unique values in a stream")
    .arg(Arg::with_name("size")
      .short("s")
      .long("size")
      .help("Set the size of buffer (in 4 byte words)")
      .default_value("100000")
      .takes_value(true));

  App::new("Fast estimate").subcommands(vec![top, uniq])
}
