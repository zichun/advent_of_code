use itertools::Itertools;
use std::{collections::{HashMap, BTreeSet}, ops::{RangeFrom, RangeBounds}};

#[derive(Default, Debug)]
struct Map {
    objects: HashMap<i32, BTreeSet<i32>>,
    floor: Option<i32>,
}

impl Map {
    fn add_object(&mut self, r: i32, c: i32) {
        self.objects.entry(c).or_insert(BTreeSet::new())
            .insert(r);
    }
    fn set_floor(&mut self, floor: i32) {
        self.floor = Some(floor);
    }
    fn has_object(&self, r: i32, c: i32) -> bool {
        if !self.objects.contains_key(&c) {
            if let Some(floor) = self.floor {
                floor == r
            } else {
                false
            }
        } else {
            match self.objects[&c].range(r..).next() {
                None => false,
                Some(next_r) => {
                    r == *next_r
                }
            }
        }
    }
    fn drop_sand(&mut self, r: i32, c: i32) -> bool {
        if !self.objects.contains_key(&c) {
            if let Some(floor) = self.floor {
                self.add_object(floor - 1, c);
                true
            } else {
                false
            }
        } else {
            match self.objects[&c].range(r..).next() {
                Some(r) => {
                    let r = *r - 1;
                    if !self.has_object(r + 1, c - 1) {
                        self.drop_sand(r + 1, c - 1)
                    } else if !self.has_object(r + 1, c + 1) {
                        self.drop_sand(r + 1, c + 1)
                    } else {
                        self.add_object(r, c);
                        true
                    }
                },
                None => if let Some(floor) = self.floor {
                    self.add_object(floor - 1, c);
                    true
                } else {
                    false
                },
            }
        }
    }
}

fn parse(input: &str) -> (Map, i32) {
    let mut map = Map::default();
    let mut floor = 0;
    input.lines()
        .for_each(|l|
                  l.split(" -> ").map(|chunk|
                                      chunk.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>())
                  .tuple_windows()
                  .for_each(|(from, to)| {
                      floor = floor.max(from[1].max(to[1]));
                      if from[0] == to[0] {
                          for r in from[1].min(to[1])..=from[1].max(to[1]) {
                              map.add_object(r, from[0]);
                          }
                      } else {
                          for c in from[0].min(to[0])..=from[0].max(to[0]) {
                              map.add_object(from[1], c);
                          }
                      }
                  }));
    (map, floor)
}

pub fn part2(input: &str) -> usize {
    let (mut map, floor) = parse(input);
    map.set_floor(floor + 2);
    for sand_cnt in 0.. {
        assert!(map.drop_sand(0, 500));
        if map.has_object(0, 500) {
            return sand_cnt + 1;
        }
    }
    0
}

pub fn part1(input: &str) -> usize {
    let (mut map, _) = parse(input);
    for sand_cnt in 0.. {
        if !map.drop_sand(0, 500) {
            return sand_cnt;
        }
    }
    0
}

#[test]
fn test() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    assert_eq!(part1(input), 24);
    assert_eq!(part2(input), 93);
}
