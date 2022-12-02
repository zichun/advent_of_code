use std::collections::HashMap;

struct Recitation {
    input: Vec<u32>,
    turn: usize,
    prev: u32,
    last_spoken: Vec<Option<u32>>,
}

impl Iterator for Recitation {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let tr = if self.turn < self.input.len() {
            self.input[self.turn]
        } else {
            if self.prev >= self.last_spoken.len() as u32 ||
                self.last_spoken[self.prev as usize] == None
            {
                0
            } else if let Some(v) = self.last_spoken[self.prev as usize] {
                self.turn as u32 - v - 1
            } else {
                panic!("Unexpected");
            }
        };
        if self.turn > 0 {
            if self.prev >= self.last_spoken.len() as u32 {
                self.last_spoken.resize(self.prev as usize + 1, None);
            }
            self.last_spoken[self.prev as usize] = Some((self.turn - 1) as u32);
        }
        self.prev = tr;
        self.turn += 1;
        Some(tr)
    }
}

impl Recitation {
    fn new(inp: &[u32]) -> Self {
        Recitation {
            input: inp.to_owned(),
            turn: 0,
            prev: 0,
            last_spoken: Vec::new()
        }
    }
}

pub fn day15_1(input: &str) -> u32 {
    let input = input.lines().next().unwrap().split(',')
        .filter(|&x| !x.trim().is_empty())
        .map(|x|
             x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    Recitation::new(&input).skip(2019).next().unwrap()
}

pub fn day15_2(input: &str) -> u32 {
    let input = input.lines().next().unwrap().split(',')
        .filter(|&x| !x.trim().is_empty())
        .map(|x|
             x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    Recitation::new(&input).skip(30000000 - 1).next().unwrap()
}

#[test]
fn test_day15_1() {
    let input = "0,3,6";
    assert_eq!(day15_1(input), 436);
}
