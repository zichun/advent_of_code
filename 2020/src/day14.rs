////
//// PART 1
////

use std::collections::HashMap;

struct Mask {
    and: u64,
    or: u64,
    mask: Vec<usize>
}

struct MaskIter {
    base: u64,
    ind: usize,
    mask: Vec<usize>
}

impl Iterator for MaskIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ind >= 1 << self.mask.len() {
            None
        } else {
            let mut tr = self.base;
            for i in 0..self.mask.len() {
                if (self.ind as u64) & (1 << i) > 0 {
                    tr |= 1 << self.mask[i];
                }
                else
                {
                    tr &= !(1 << self.mask[i]);
                }
            }
            self.ind += 1;
            Some(tr)
        }
    }
}

impl Mask {
    fn from(str: &str) -> Self {
        let mut mask = Vec::new();
        let (or, nand) = str.chars()
            .enumerate()
            .fold((0, 0), |acc, (ind, el)| {
                match el {
                    'X' => {
                        mask.push(str.len() - ind - 1);
                        (acc.0 << 1, acc.1 << 1)
                    }
                    '0' => {
                        (acc.0 << 1, (acc.1 << 1) | 1)
                    },
                    '1' => {
                        ((acc.0 << 1) | 1, acc.1 << 1)
                    },
                    _ => panic!("Invalid input")
                }
            });

        Mask {
            and: !nand,
            or,
            mask
        }
    }
    fn apply(&self, val: u64) -> u64 {
        (val & self.and) | self.or
    }
    fn apply_part2(&self, val: u64) -> MaskIter {
        let base = val | self.or;
        MaskIter {
            base,
            ind: 0,
            mask: self.mask.clone()
        }
    }
}

pub fn day14_1(input: &str) -> u64 {
    let mut map = HashMap::new();
    input.lines()
        .fold(None,
              |mask, el| {
                  let mut inp = el.split(" = ");
                  let first = inp.next().unwrap();
                  let val = inp.next().unwrap();
                  if first.starts_with("mask") {
                      let mask = Mask::from(val);
                      Some(mask)
                  } else {
                      let val = val.parse::<u64>().unwrap();
                      let key = first.chars().skip(4).take(first.len() - 5).collect::<String>().parse::<u64>().unwrap();
                      if let Some(m) = mask {
                          map.insert(key, m.apply(val));
                          Some(m)
                      } else {
                          panic!("Set without mask");
                      }
                  }
              });
    map.iter().map(|(_, val)| val).sum()
}

pub fn day14_2(input: &str) -> u64 {
    let mut map = HashMap::new();
    input.lines()
        .fold(None,
              |mask, el| {
                  let mut inp = el.split(" = ");
                  let first = inp.next().unwrap();
                  let val = inp.next().unwrap();
                  if first.starts_with("mask") {
                      let mask = Mask::from(val);
                      Some(mask)
                  } else {
                      let val = val.parse::<u64>().unwrap();
                      let key = first.chars().skip(4).take(first.len() - 5).collect::<String>().parse::<u64>().unwrap();
                      if let Some(m) = mask {
                          m.apply_part2(key).for_each(|k| {
                              map.insert(k, val);
                          });
                          Some(m)
                      } else {
                          panic!("Set without mask");
                      }
                  }
              });
    map.iter().map(|(_, val)| val).sum()
}

#[test]
fn test_day14_1() {
    let inp = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    assert_eq!(day14_1(inp), 165);
}

#[test]
fn test_day14_2() {
    let inp = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
    assert_eq!(day14_2(inp), 208);
}
