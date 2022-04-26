use utils::timer::Timer;

mod day1_no_time_for_taxicab;
mod day2_bathroom_security;
mod day3_squares_with_three_sides;
mod day4_security_through_obscurity;
mod day5_game_of_chess;
mod day6_signals_and_noise;
mod day7_internet_protocol_v7;
mod day8_two_factor_authentication;
mod day9_explosives_in_cyberspace;
mod day10_balance_bots;
mod day11_radioisotope_thermoelectric_generators;
mod day12_leonardos_monorail;
mod day13_a_maze_of_twisty_little_cubicles;

fn main() {
    let _timer = Timer::start(|elapsed| println!("main took {} ms.", elapsed.as_millis()));
    let day: usize = if let Some(arg1) = std::env::args().nth(1) {
        arg1.parse().expect("argument should be an integer")
    } else {
        13
    };
    println!("running day {}\n", day);
    match day {
        1 => day1_no_time_for_taxicab::run(),
        2 => day2_bathroom_security::run(),
        3 => day3_squares_with_three_sides::run(),
        4 => day4_security_through_obscurity::run(),
        5 => day5_game_of_chess::run(),
        6 => day6_signals_and_noise::run(),
        7 => day7_internet_protocol_v7::run(),
        8 => day8_two_factor_authentication::run(),
        9 => day9_explosives_in_cyberspace::run(),
        10 => day10_balance_bots::run(),
        11 => day11_radioisotope_thermoelectric_generators::run(),
        12 => day12_leonardos_monorail::run(),
        13 => day13_a_maze_of_twisty_little_cubicles::run(),
        _ => panic!("day {} not found", day)
    }
}
