use anyhow::{anyhow, Result};
use log::*;

pub(crate) fn run() {
    let _input = 5;
    let _input = 301845;
    let _input = 3018458;

    let elves = vec![Elf::new(); _input];
    println!("\ngetting to the left:");
    exchange_presents(elves.clone(), get_next_to_left).unwrap();

    println!("\ngetting across:");
    // exchange_presents(elves, _get_next_across).unwrap();

    _exchange_presents_v2(elves).unwrap();
}

fn get_next_to_left(elves: &[Elf], i: usize, _remaining: usize) -> Option<usize> {
    let n = elves.len();
    (1..n).map(|j| (j + i) % n)
        .filter(|&j| elves[j].presents > 0).next()
}

// takes too long...
fn _get_next_across(elves: &[Elf], i: usize, remaining: usize) -> Option<usize> {
    let n = elves.len();
    (1..n).map(|j| (j + i) % n)
        .filter(|&j| elves[j].presents > 0).nth((remaining / 2).max(1) - 1)
}

fn exchange_presents<TFun: Fn(&[Elf], usize, usize) -> Option<usize>>(mut elves: Vec<Elf>, get_next_elf: TFun) -> Result<()> {
    let n = elves.len();
    let mut i = 0usize;
    let mut remaining_elves = n;
    loop {
        let other_elf = match get_next_elf(&elves, i, remaining_elves) {
            None => { break; }
            Some(other) => other
        };
        let presents = std::mem::take(&mut elves[other_elf].presents);
        debug!("elf {} steals presents from elf {}", i + 1, other_elf + 1);
        elves[i].presents += presents;
        loop {
            i = (i + 1) % n;
            if elves[i].presents > 0 {
                break;
            }
        }
        remaining_elves -= 1;
        if remaining_elves % ((n / 20).max(1)) == 0 {
            info!("{} elves remain ({}%)", remaining_elves, (n - remaining_elves) * 100 / n);
        }
    }

    let winner = elves.iter().enumerate().filter(|(_, e)| e.presents > 0).next().ok_or(anyhow!("no winner found..."))?;
    println!("winning elf is {} with {} presents", winner.0 + 1, winner.1.presents);
    Ok(())
}

fn _exchange_presents_v2(mut elves: Vec<Elf>) -> Result<()> {
    let mut i = 0usize;
    let mut remaining_elves = elves.len();
    let mut opposite = elves.len() / 2;
    let mut remainder = elves.len() % 2;
    let get_next = |elves: &[Elf], mut i: usize| {
        loop {
            i = (i + 1) % elves.len();
            if elves[i].presents > 0 {
                break;
            }
        }
        i
    };
    loop {
        let other_elf = opposite;

        let presents = std::mem::take(&mut elves[other_elf].presents);
        debug!("elf {} steals presents from elf {}", i + 1, other_elf + 1);
        elves[i].presents += presents;

        opposite = get_next(&elves, opposite);
        if remainder == 1 {
            opposite = get_next(&elves, opposite);
            remainder = 0;
        } else {
            remainder = 1;
        }

        remaining_elves -= 1;
        if remaining_elves == 1 {
            break;
        }

        i = get_next(&elves, i);

        if remaining_elves % ((elves.len() / 20).max(1)) == 0 {
            info!("{} elves remain ({}%)", remaining_elves, (elves.len() - remaining_elves) * 100 / elves.len());
        }
    }

    let winner = elves.iter().enumerate().filter(|(_, e)| e.presents > 0).next().ok_or(anyhow!("no winner found..."))?;
    println!("winning elf is {} with {} presents", winner.0 + 1, winner.1.presents);
    Ok(())
}

#[derive(Clone)]
struct Elf {
    presents: usize,
}

impl Elf {
    pub(crate) fn new() -> Self {
        Self { presents: 1 }
    }
}