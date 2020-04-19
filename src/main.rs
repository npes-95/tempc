use std::env;
use std::process;

use tempc::cfg;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = cfg::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = tempc::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    } 
}
