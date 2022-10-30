mod cli;
mod generator;
mod champions;

use crate::generator::{get_random_champions};
use std::env;

fn main() {
    // cli::run();
    
    let args: Vec<String> = env::args().collect();
    let champions = get_random_champions(args[1].parse::<usize>().unwrap());
    println!("{champions:?}");
}
