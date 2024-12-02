use std::env;

mod day01;
mod day02;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).expect("First argument should be the day");
    let day = day.parse::<u8>().expect("Day must be a number");

    match day {
        1 => {
            day01::part_one();
            day01::part_two();
        }
        2 => {
            day02::part_one();
            day02::part_two();
        }
        _ => println!("Day not implemented yet"),
    }
}
