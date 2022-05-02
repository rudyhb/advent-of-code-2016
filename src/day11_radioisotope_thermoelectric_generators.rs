use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use utils::a_star::*;

const NUM_FLOORS: usize = 4;

pub(crate) fn run() {
    let _input = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";
    let _input = _get_input();

    let mut building: Building = _input.into();
    add_extra_items_on_first_floor(&mut building, true);

    let target = {
        let mut last_floor = building
            .floors
            .iter()
            .flat_map(|f| f.iter().cloned())
            .collect::<Vec<_>>();
        last_floor.sort();

        Building {
            elevator_floor: 3,
            floors: [
                Default::default(),
                Default::default(),
                Default::default(),
                last_floor,
            ],
        }
    };
    println!("target state:{}", target);

    println!("initial state:{}\n", building);

    let solution = a_star_search(building, &target, get_successors, distance_function, None)
        .expect("no solution found");
    for (i, step) in solution.iter().enumerate().skip(1) {
        println!("step {}:{}\n", i, step);
    }
    println!("{} steps needed", solution.len() - 1);
}

fn add_extra_items_on_first_floor(building: &mut Building, add: bool) {
    if !add {
        return;
    }
    building.floors[0].extend([
        Device::Generator("elerium"),
        Device::Microchip("elerium"),
        Device::Generator("dilithium"),
        Device::Microchip("dilithium"),
    ])
}

impl AStarNode for Building {}

fn get_limited_items(from: &[Device]) -> Option<Vec<Device>> {
    if from.len() < 5 {
        return None;
    }
    let mut map: HashMap<&str, Vec<&Device>> = Default::default();
    for device in from {
        let name = device.get_name();
        map.entry(name).or_default().push(device);
    }

    let mut count_both = 0usize;
    Some(
        map.into_iter()
            .flat_map(|(_, devices)| {
                if devices.len() == 2 {
                    if count_both < 2 {
                        count_both += 1;
                        devices
                    } else {
                        Default::default()
                    }
                } else {
                    devices
                }
            })
            .cloned()
            .collect(),
    )
}

fn get_successors(state: &Building) -> Vec<Successor<Building>> {
    let mut items = &state.floors[state.elevator_floor];
    let limited_items = get_limited_items(items);
    if let Some(value) = &limited_items {
        items = value;
    }
    let items = items;

    let possible_floors = if state.elevator_floor == NUM_FLOORS - 1 {
        vec![state.elevator_floor - 1]
    } else if state.elevator_floor == 0 {
        vec![1]
    } else {
        vec![state.elevator_floor - 1, state.elevator_floor + 1]
    };
    possible_floors
        .into_iter()
        .flat_map(|to_floor| {
            if items.len() < 2 {
                return items
                    .into_iter()
                    .filter(|&item| state.is_valid_bringing(&[item], to_floor))
                    .map(|item| {
                        let mut next = state.clone();
                        next.bring(vec![item.clone()], to_floor);
                        Successor::new(next, 1)
                    })
                    .collect::<Vec<_>>();
            }
            permutator::Combination::combination(items, 2)
                .chain(items.iter().map(|item| vec![item]))
                .filter(move |c| state.is_valid_bringing(&c[..], to_floor))
                .map(move |c| {
                    let mut next = state.clone();
                    next.bring(c.into_iter().cloned().collect(), to_floor);
                    Successor::new(next, 1)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn distance_function(details: CurrentNodeDetails<Building>) -> i32 {
    let n = details.current_node.floors.len();
    10 * (0..n - 1).fold(0i32, |distance, i| {
        let count = details.current_node.floors[i].len();
        distance + (count * (n - i - 1)) as i32
    }) + details
        .current_node
        .floors
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut map, (i, devices)| {
            for device in devices {
                let entry: &mut usize = map.entry(device.get_name()).or_default();
                *entry = (*entry).max(i) - (*entry).min(i);
            }
            map
        })
        .into_iter()
        .map(|(_, diff)| diff as i32)
        .sum::<i32>()
}

#[derive(Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Building {
    elevator_floor: usize,
    floors: [Vec<Device>; NUM_FLOORS],
}

impl Display for Building {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;
        for (i, floor) in self
            .get_ordered_row_contents()
            .into_iter()
            .enumerate()
            .rev()
        {
            write!(
                f,
                "F{i} {elevator} {floor}\n",
                i = i + 1,
                elevator = if self.elevator_floor == i { 'E' } else { '.' },
                floor = floor
                    .into_iter()
                    .map(|space| match space {
                        None => ". ".to_string(),
                        Some(device) => match device {
                            Device::Generator(g) =>
                                format!("{}G", g.chars().next().unwrap().to_uppercase()),
                            Device::Microchip(m) =>
                                format!("{}M", m.chars().next().unwrap().to_uppercase()),
                        },
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            )?
        }
        Ok(())
    }
}

impl Building {
    pub(crate) fn is_valid_bringing(&self, items: &[&Device], to_floor: usize) -> bool {
        let floor = &self.floors[to_floor];
        let from_floor = &self.floors[self.elevator_floor];
        for item in items {
            match item {
                Device::Generator(generator) => {
                    if floor.iter().any(|device| {
                        if let Device::Microchip(name) = device {
                            if name != generator {
                                let other_generator = Device::Generator(name);
                                if !items.contains(&&other_generator)
                                    && !floor.contains(&other_generator)
                                {
                                    return true;
                                }
                            }
                        }
                        false
                    }) {
                        return false;
                    }
                    let chip = Device::Microchip(generator);
                    if !items.contains(&&chip) && from_floor.contains(&chip) {
                        if from_floor
                            .iter()
                            .filter(|item| !items.contains(item))
                            .any(|item| {
                                if let Device::Generator(name) = item {
                                    name != generator
                                } else {
                                    false
                                }
                            })
                        {
                            return false;
                        }
                    }
                }
                Device::Microchip(chip) => {
                    let generator = Device::Generator(chip);
                    //                     println!("\
                    // chip: {chip:?}
                    // generator: {generator:?}
                    // items: {items:?}
                    // floor: {floor:?}",
                    //                     chip=chip,
                    //                     generator=generator,
                    //                     items=items,
                    //                     floor=floor);
                    if !items.contains(&&generator) && !floor.contains(&generator) {
                        if floor.iter().chain(items.iter().copied()).any(|item| {
                            if let Device::Generator(_) = item {
                                true
                            } else {
                                false
                            }
                        }) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
    pub(crate) fn bring(&mut self, items: Vec<Device>, to_floor: usize) {
        self.floors[self.elevator_floor].retain(|i| !items.contains(i));
        self.floors[self.elevator_floor].sort();
        self.floors[to_floor].extend(items);
        self.floors[to_floor].sort();
        self.elevator_floor = to_floor;
    }
    pub(crate) fn get_ordered_row_contents(&self) -> [Vec<Option<&Device>>; NUM_FLOORS] {
        let mut all: Vec<_> = self.floors.iter().flat_map(|f| f.iter()).collect();
        all.sort();
        let result: Vec<_> = self
            .floors
            .iter()
            .map(|f| {
                all.iter()
                    .map(|&device| {
                        if f.contains(device) {
                            Some(device)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Option<&Device>>>()
            })
            .collect();
        result.try_into().unwrap()
    }
}

impl From<&'static str> for Building {
    fn from(s: &'static str) -> Self {
        let clean_word = |word: &'static str| word.split('-').next().unwrap();
        let parse_floor = |floor: &'static str| {
            let mut devices: Vec<Device> = Default::default();
            let mut last_word = "";
            for word in floor.split_whitespace() {
                match word.trim_end_matches(&[',', '.']) {
                    "generator" => {
                        devices.push(Device::Generator(last_word));
                    }
                    "microchip" => {
                        devices.push(Device::Microchip(clean_word(last_word)));
                    }
                    _ => {}
                }
                last_word = word;
            }

            devices.sort();
            devices
        };
        Self {
            elevator_floor: 0,
            floors: s
                .lines()
                .map(|line| parse_floor(line))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum Device {
    Generator(&'static str),
    Microchip(&'static str),
}

impl Device {
    pub(crate) fn get_name(&self) -> &str {
        match self {
            Device::Generator(name) => name,
            Device::Microchip(name) => name,
        }
    }
}

fn _get_input() -> &'static str {
    "The first floor contains a polonium generator, a thulium generator, a thulium-compatible microchip, a promethium generator, a ruthenium generator, a ruthenium-compatible microchip, a cobalt generator, and a cobalt-compatible microchip.
The second floor contains a polonium-compatible microchip and a promethium-compatible microchip.
The third floor contains nothing relevant.
The fourth floor contains nothing relevant."
}
