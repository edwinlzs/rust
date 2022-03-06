use std::env;
use std::process;

use minigrep::Config;

fn main() {
  let args: Vec<String> = env::args().collect(); // first value in vector is name of binary

  let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err); // prints to stderror instead of stdout
    process::exit(1);
  });

  // we dont need to unwrap any value for run() - only need to detect error
  if let Err(e) = minigrep::run(config) {
    eprintln!("Application error: {}", e);
    
    process::exit(1);
  }
}

