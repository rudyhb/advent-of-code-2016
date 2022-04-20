use utils::timer::Timer;

mod day1_no_time_for_taxicab;
mod day2_bathroom_security;
mod day3_squares_with_three_sides;
mod day4_security_through_obscurity;

fn main() {
    let _timer = Timer::start(|elapsed| println!("main took {} ms.", elapsed.as_millis()));
    let day: usize = if let Some(arg1) = std::env::args().nth(1) {
        arg1.parse().expect("argument should be an integer")
    } else {
        4
    };
    println!("running day {}\n", day);
    match day {
        1 => day1_no_time_for_taxicab::run(),
        2 => day2_bathroom_security::run(),
        3 => day3_squares_with_three_sides::run(),
        4 => day4_security_through_obscurity::run(),
        _ => panic!("day {} not found", day)
    }
}
