use std::io::Write;

use crate::champions::CHAMPIONS;
use crate::generator::get_random_champions_from_pool;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    amount: Option<usize>,

    #[arg(short, long, value_name = "CHAMPION1,CHAMPION2")]
    champions: Option<String>,
}

pub fn run() {
    let args = Args::parse();

    let champion_pool = if let Some(champions) = &args.champions {
        champions.split(',').collect::<Vec<&str>>()
    } else {
        CHAMPIONS.to_vec()
    };

    let amount = if let Some(amount) = args.amount {
        amount
    }
    else {
        print!("Number of champions to generate: "); 
        std::io::stdout().flush().unwrap();
        let line = std::io::stdin().lines().next().unwrap().unwrap(); 
        line.parse::<usize>().unwrap()
    };

    if amount > champion_pool.len() {
        println!("Amount of random champions {{{}}} can't be greater than the size of the specified champion pool {{{}}}!", amount, champion_pool.len());
        return;
    }

    let random_champions = get_random_champions_from_pool(amount, &champion_pool);
    println!("{}", random_champions.join("\n"));
}
