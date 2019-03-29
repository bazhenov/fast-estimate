#[macro_use]
extern crate clap;
pub mod linear_counter;
pub mod stream_summary;
pub mod double_linked_list;

use clap::{Arg, App, SubCommand};
use std::io;

use stream_summary::StreamSummary;
use linear_counter::LinearCounter;

use std::process::exit;

fn build_cli() -> App<'static, 'static> {
  let top = SubCommand::with_name("top")
    .about("Estimates a top-k values in a stream")
    .arg(Arg::with_name("size")
      .long("size")
      .short("s")
      .takes_value(true)
      .default_value("1000")
      .help("Capacity of a stream-summary"));

  let uniq = SubCommand::with_name("uniq")
    .about("Estimate number of unique values in a stream")
    .arg(Arg::with_name("size")
      .short("s")
      .long("size")
      .help("Set the size of buffer (in 4 byte words)")
      .default_value("100000")
      .takes_value(true));

  App::new("Fast estimate")
    .arg(Arg::with_name("help")
      .long("help")
      .short("h")
      .help("Show this help"))
    .subcommands(vec![top, uniq])
}

fn usage() {
  build_cli().write_long_help(&mut io::stdout()).expect("Failed to write help");
  exit(0);
}

fn main() {
  let matches = build_cli().get_matches();

  if matches.is_present("help") {
    usage();
  }

  if let Some(matches) = matches.subcommand_matches("uniq") {
    let size = value_t!(matches, "size", usize).unwrap_or_else(|e| e.exit());
    let mut lc = LinearCounter::new(size);

    stdin_line_loop(|line| lc.offer(line));

    println!("{:}", lc.estimate())

  } else if let Some(matches) = matches.subcommand_matches("top") {
    let size = value_t!(matches, "size", usize).unwrap_or_else(|e| e.exit());
    let mut summary = StreamSummary::with_capacity(size);

    stdin_line_loop(|line| {summary.offer(line);});

    for item in summary.estimate_top() {
      println!("{:6} : {}", item.count, item.data)
    }

  } else {
    usage();
  }
}

fn trim_newline(s: &mut String) {
  if s.ends_with('\n') {
    s.pop();
    if s.ends_with('\r') {
      s.pop();
    }
  }
}

fn stdin_line_loop<F>(mut line_callback: F)
  where F: FnMut(&str) {

  let stdin = io::stdin();
  let mut line = String::new();

  loop {
    line.clear();
    match stdin.read_line(&mut line) {
      Ok(n) if n <= 0 => break,
      Ok(_) => {
        trim_newline(&mut line);
        line_callback(&line)
      },
      Err(e) => panic!("{:?}", e)
    }
  }
}
