use std::collections::HashSet;

pub(crate) fn run() {
    let _input = "abc";
    let _input = "cuanljph";

    let mut computer = Computer::new(_input);
    let n = 64usize;
    computer.get_keys(n, false);
    // println!("{:?}", computer.keys.iter().filter(|k| k.resolved).map(|k| k.index).collect::<Vec<_>>());
    println!("Using salt of {} index {} produces the {}th key", computer.salt, computer.keys.iter().nth(n - 1).unwrap().index, n);
    println!("stretching the key:");
    let mut computer = Computer::new(_input);
    computer.get_keys(n, true);
    println!("Using salt of {} index {} produces the {}th key", computer.salt, computer.keys.iter().nth(n - 1).unwrap().index, n);
}

struct Computer {
    salt: &'static str,
    current_index: usize,
    keys: Vec<Key>,
}

impl Computer {
    pub(crate) fn new(salt: &'static str) -> Self {
        Self {
            salt,
            current_index: 0,
            keys: vec![],
        }
    }
    pub(crate) fn get_keys(&mut self, count: usize, stretch: bool) {
        while self.keys.len() < count || self.keys.iter().take(count).any(|k| !k.resolved)
        {
            self.next(stretch);
        }
    }
    fn next(&mut self, stretch: bool) {
        let stream: Vec<char> = {
            let s = format!("{}{}", self.salt, self.current_index);
            if stretch {
                stretched_md5(&s, 2016)
            } else {
                md5(&s)
            }
        }.chars().collect();
        let quintuples: Vec<_> = self.keys.iter().filter(|k| !k.resolved)
            .map(|k| k.triplet)
            .collect::<HashSet<_>>()
            .into_iter()
            .filter(|c| {
                stream.windows(5)
                    .any(|chars| chars[0] == *c && chars[0] == chars[1] && chars[1] == chars[2] && chars[2] == chars[3] && chars[3] == chars[4])
            })
            .collect();
        for c in quintuples.into_iter() {
            for key in self.keys.iter_mut().filter(|k| !k.resolved && k.triplet == c) {
                key.resolved = true;
                // println!("key resolved: {:?} by {}", key, self.current_index);
            }
        }

        self.keys.retain(|k| {
            let keep = k.resolved || self.current_index - k.index < 1000;
            if !keep {
                // println!("removing key {:?}", k);
            }
            keep
        });

        if let Some(c) = stream.windows(3)
            .filter(|chars| chars[0] == chars[1] && chars[1] == chars[2])
            .map(|chars| chars[0])
            .next() {
            self.keys.push(Key {
                index: self.current_index,
                triplet: c,
                resolved: false,
            })
        }

        self.current_index += 1;
    }
}

#[derive(Debug)]
struct Key {
    index: usize,
    triplet: char,
    resolved: bool,
}

fn md5(s: &str) -> String {
    let digest = md5::compute(s);
    format!("{:x}", digest)
}

fn stretched_md5(s: &str, times: usize) -> String {
    let mut digest = md5::compute(s);
    for _ in 0..times {
        digest = md5::compute(format!("{:x}", digest));
    }
    format!("{:x}", digest)
}