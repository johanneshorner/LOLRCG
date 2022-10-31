use crate::champions::CHAMPIONS;
use crate::generator::get_random_champions_from_pool;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    amount: usize,

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

    if args.amount > champion_pool.len() {
        println!("Amount of random champions {{{}}} can't be greater than the size of the specified champion pool {{{}}}!", args.amount, champion_pool.len());
        return;
    }

    let random_champions = get_random_champions_from_pool(args.amount, &champion_pool);
    println!("{}", random_champions.join("\n"));
}
