mod days;

fn main() {
    let day: usize = std::env::args()
        .nth(1)
        .expect("Please specify a day")
        .parse()
        .expect("Day must be an integer");

    println!("Day {}", day);

    match day {
        1 => days::day01::main(),
        2 => days::day02::main(),
        3 => days::day03::main(),
        4 => days::day04::main(),
        5 => days::day05::main(),
        6 => days::day06::main(),
        7 => days::day07::main(),
        8 => days::day08::main(),
        _ => panic!("Solution missing for day {}", day),
    }
}
