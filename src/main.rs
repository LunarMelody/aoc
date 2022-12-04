mod day1;
mod day2;
mod day3;
mod day4;

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
        3 => {
            day3::solution::part1();
            day3::solution::part2();
        },
        4 => {
            day4::solution::part1();
            day4::solution::part2();
        },
        _ => panic!("I didn't solve this :(")
    }
}
