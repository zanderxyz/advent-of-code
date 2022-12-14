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
        9 => days::day09::main(),
        10 => days::day10::main(),
        11 => days::day11::main(),
        12 => days::day12::main(),
        13 => days::day13::main(),
        14 => days::day14::main(),
        // 15 => days::day15::main(),
        // 16 => days::day16::main(),
        // 17 => days::day17::main(),
        // 18 => days::day18::main(),
        // 19 => days::day19::main(),
        // 20 => days::day20::main(),
        // 21 => days::day21::main(),
        // 22 => days::day22::main(),
        // 23 => days::day23::main(),
        // 24 => days::day24::main(),
        // 25 => days::day25::main(),
        _ => panic!("Solution missing for day {}", day),
    }
}
