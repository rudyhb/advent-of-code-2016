pub(crate) fn run() {
    let _input = "abc";
    let _input = _get_input();

    // let mut hacker = Hacker::new(_input);
    let mut hacker = HackerV2::new(_input);

    let n = 8;
    for i in 0..n {
        hacker.get_next();
        println!("{} %", (i + 1) * 100 / n);
    }
    println!("password is {}", hacker.get_password());
}

struct Hacker<'a> {
    door_id: &'a str,
    nonce: usize,
    password: String,
}

#[allow(unused)]
impl<'a> Hacker<'a> {
    const LEADING_ZEROS: usize = 5;
    pub(crate) fn new(door_id: &'a str) -> Self {
        Self {
            door_id,
            nonce: 0,
            password: String::new(),
        }
    }
    pub(crate) fn get_next(&mut self) {
        let leader = &"0".repeat(Self::LEADING_ZEROS);
        loop {
            let digest = md5::compute(format!("{}{}", self.door_id, self.nonce));
            self.nonce += 1;
            let hash = format!("{:x}", digest);
            if hash.starts_with(leader) {
                self.password.push(hash.chars().nth(Self::LEADING_ZEROS).unwrap());
                break;
            }

            if self.nonce % 1_000_000 == 0 {
                println!("nonce: {}", self.nonce);
            }
        }
    }
    pub(crate) fn get_password(&self) -> &str {
        &self.password
    }
}

struct HackerV2<'a> {
    door_id: &'a str,
    nonce: usize,
    password: [Option<char>; 8],
}

impl<'a> HackerV2<'a> {
    const LEADING_ZEROS: usize = 5;
    pub(crate) fn new(door_id: &'a str) -> Self {
        Self {
            door_id,
            nonce: 0,
            password: [None; 8],
        }
    }
    pub(crate) fn get_next(&mut self) {
        if self.password.iter().all(|p| p.is_some()) {
            panic!("already resolved password");
        }
        let leader = &"0".repeat(Self::LEADING_ZEROS);
        loop {
            let digest = md5::compute(format!("{}{}", self.door_id, self.nonce));
            self.nonce += 1;
            let hash = format!("{:x}", digest);
            if hash.starts_with(leader) {
                let position = hash.chars().nth(Self::LEADING_ZEROS).unwrap().to_digit(16).unwrap() as usize;
                if position < 8 && self.password[position].is_none() {
                    let c = hash.chars().nth(Self::LEADING_ZEROS + 1).unwrap();
                    self.password[position] = Some(c);
                    break;
                }
            }

            if self.nonce % 1_000_000 == 0 {
                println!("nonce: {}", self.nonce);
            }
        }
    }
    pub(crate) fn get_password(&self) -> String {
        self.password.iter().map(|c| c.ok_or(())).collect::<Result<String, _>>().unwrap()
    }
}

fn _get_input() -> &'static str {
    "uqwqemis"
}