use utils::timer::Timer;

mod day10_balance_bots;
mod day11_radioisotope_thermoelectric_generators;
mod day12_leonardos_monorail;
mod day13_a_maze_of_twisty_little_cubicles;
mod day14_one_time_pad;
mod day15_timing_is_everything;
mod day16_dragon_checksum;
mod day17_two_steps_forward;
mod day18_like_a_rogue;
mod day19_an_elephant_named_joseph;
mod day1_no_time_for_taxicab;
mod day20_firewall_rules;
mod day21_scrambled_letters_and_hash;
mod day2_bathroom_security;
mod day3_squares_with_three_sides;
mod day4_security_through_obscurity;
mod day5_game_of_chess;
mod day6_signals_and_noise;
mod day7_internet_protocol_v7;
mod day8_two_factor_authentication;
mod day9_explosives_in_cyberspace;

fn main() {
    env_logger::init();
    let _timer = Timer::start(|elapsed| println!("main took {} ms.", elapsed.as_millis()));
    let day: usize = if let Some(arg1) = std::env::args().nth(1) {
        arg1.parse().expect("argument should be an integer")
    } else {
        21
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
        14 => day14_one_time_pad::run(),
        15 => day15_timing_is_everything::run(),
        16 => day16_dragon_checksum::run(),
        17 => day17_two_steps_forward::run(),
        18 => day18_like_a_rogue::run(),
        19 => day19_an_elephant_named_joseph::run(),
        20 => day20_firewall_rules::run(),
        21 => day21_scrambled_letters_and_hash::run(),
        _ => panic!("day {} not found", day),
    }
}
