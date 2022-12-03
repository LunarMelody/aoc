mod day1;
mod day2;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long, default_value_t = 1)]
   day: u8,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => {
            day1::solution::part1();
            day1::solution::part2();
        },
        2 => {
            day2::solution::part1();
            day2::solution::part2();
        },
        _ => panic!("I didn't solve this :(")
    }
}
