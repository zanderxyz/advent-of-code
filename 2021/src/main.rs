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
        _ => panic!("Solution missing for day {}", day),
    }
}
