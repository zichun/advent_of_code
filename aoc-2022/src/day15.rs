use regex::Regex;
use std::collections::{HashMap, BTreeSet};

type Sensors = Vec<(i32, i32, i32)>;
type Beacons = HashMap<i32, BTreeSet<i32>>;

fn parse(input: &str) -> (Sensors, Beacons) {
    let mut s = Vec::new();
    let mut b = HashMap::new();
    input.lines().for_each(|l| {
        let re = Regex::new("Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)").unwrap();
        if let Some(cap) =  re.captures(l)  {
            let sensor: (i32, i32) = (cap[2].parse().unwrap(),
                          cap[1].parse().unwrap());
            let beacon: (i32, i32) = (cap[4].parse().unwrap(),
                          cap[3].parse().unwrap());
            let diff = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
            s.push((sensor.0, sensor.1, diff));
            b.entry(beacon.0).or_insert(BTreeSet::new()).insert(beacon.1);
        }
    });
    s.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    (s, b)
}

fn part1_inner(input: &str, y: i32) -> i32 {
    let (s, mut b) = parse(input);
    let mut ranges = Vec::<(i32, i32)>::new();
    s.iter().for_each(|(r, c, rad)| {
        let diff = *rad - (y - r).abs();
        if diff >= 0 {
            let (mut c0, mut c1) = (c - diff, c + diff);
            let mut found = false;
            ranges.iter_mut().for_each(|(l, r)| {
                if (*l..=*r).contains(&c0) {
                    *r = *r.max(&mut c1);
                    found = true;
                } else if (*l..=*r).contains(&c1) {
                    *l = *l.min(&mut c0);
                    found = true;
                }
            });
            if !found {
                ranges.push((c0, c1))
            }
        }
    });
    ranges.sort();
    let beacons = b.entry(y).or_insert(BTreeSet::new());
    ranges.iter().fold(0, |acc, (l, r)| {
        acc + r + 1 - l - beacons.range(l..r).count() as i32
    })
}

pub fn part1(input: &str) -> i32 {
    part1_inner(input, 2000000)
}

fn check_sensor(sensor: &(i32, i32, i32), r: i32, c: &mut i32) {
    let diff = sensor.2 - (r - sensor.0).abs();
    if diff >= 0 {
        let (c_left, c_right) = (sensor.1 - diff, sensor.1 + diff);
        if c_left <= *c && c_right >= *c {
            *c = c_right + 1;
        }
    }
}
fn part2_inner(input: &str, bound: i32) -> i64 {
    let (s, _) = parse(input);
    for r in 0..=bound {
        let mut c = 0;
        for i in 0..s.len() {
            check_sensor(&s[i], r, &mut c);
        }
        if c < bound {
            return c as i64 * 4000000 + r as i64;
        }
    }
    panic!("failed to find answer");
}

pub fn part2(input: &str) -> i64 {
    part2_inner(input, 4000000)
}

#[test]
fn test() {
    let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    assert_eq!(part1_inner(input, 10), 26);
    assert_eq!(part2_inner(input, 20), 56000011);
}
