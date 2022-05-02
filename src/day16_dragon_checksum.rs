use log::*;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub(crate) fn run() {
    let _input = "10000";
    let _length = 20usize;
    let _input = "01110110101001000";
    let _length = 272usize;
    let _length = 35651584usize;

    let mut data: Data = _input.parse().unwrap();
    data.expand_to(_length);
    println!("checksum: {}", data.get_checksum());
}

#[derive(Clone)]
struct Data(Vec<bool>);

impl Data {
    pub(crate) fn expand_to(&mut self, size: usize) {
        trace!("{}", self);
        while self.0.len() < size {
            let mut copy = self.clone();
            self.0.push(false);
            copy.inverse();
            self.0.extend(copy.0);
            trace!("{}", self);
        }
        self.0.truncate(size);
        trace!("{}", self);
    }
    fn inverse(&mut self) {
        self.0.reverse();
        for i in self.0.iter_mut() {
            *i = !*i;
        }
    }
    fn get_checksum(&self) -> Self {
        let mut result = Vec::with_capacity(self.0.len() / 2);
        for i in 0..self.0.len() / 2 {
            let (i, j) = (i * 2, i * 2 + 1);
            result.push(if self.0[i] == self.0[j] { true } else { false });
        }
        let result = Self(result);
        if result.0.len() % 2 == 0 {
            trace!("intermediate checksum: {}", result);
            result.get_checksum()
        } else {
            result
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|&i| if i { '1' } else { '0' })
                .collect::<String>()
        )
    }
}

impl FromStr for Data {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(|c| if c == '0' { false } else { true })
                .collect(),
        ))
    }
}
