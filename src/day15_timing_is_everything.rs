use std::str::FromStr;

pub(crate) fn run() {
    let _input = "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";
    let _input = _get_input();

    let mut disks: Vec<Disk> = _input.lines().map(|line| line.parse().unwrap()).collect();
    let time = get_first_time_to_press_button(&disks);
    println!("the first time to press the button to get a capsule is t={}", time);

    disks.push(Disk { num_positions: 11, position: 0 });
    let time = get_first_time_to_press_button(&disks);

    println!("with additional disk, the first time to press the button to get a capsule is t={}", time);
}

fn get_first_time_to_press_button(disks: &[Disk]) -> usize {
    const MAX_TIME: usize = 1_000_000_000;

    (0..MAX_TIME)
        .filter(|&t| {
            disks.iter().enumerate().all(|(i, disk)| {
                disk.position_at(t + i + 1) == 0
            })
        })
        .next()
        .expect("max time exceeded")
}

struct Disk {
    num_positions: usize,
    position: usize,
}

impl Disk {
    pub(crate) fn position_at(&self, t: usize) -> usize {
        (self.position + t) % self.num_positions
    }
}

impl FromStr for Disk {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(';');
        let num_positions: usize = {
            let mut words = parts.next().ok_or(())?.split_whitespace().rev();
            words.next();
            words.next().ok_or(())?.parse().or(Err(()))?
        };
        let position: usize = parts.next().ok_or(())?.split_whitespace().last().ok_or(())?.trim_end_matches('.').parse().or(Err(()))?;

        Ok(Self { num_positions, position })
    }
}

fn _get_input() -> &'static str {
    "Disc #1 has 13 positions; at time=0, it is at position 10.
Disc #2 has 17 positions; at time=0, it is at position 15.
Disc #3 has 19 positions; at time=0, it is at position 17.
Disc #4 has 7 positions; at time=0, it is at position 1.
Disc #5 has 5 positions; at time=0, it is at position 0.
Disc #6 has 3 positions; at time=0, it is at position 1."
}