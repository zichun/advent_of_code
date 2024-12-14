mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub mod prelude {
    pub use aoc_helper::prelude::*;
    pub use aoc_runner_derive::{aoc, aoc_generator};
    pub use itertools::{iproduct, Itertools};
    pub use regex::{Captures, Regex};
    pub use std::collections::hash_map::Entry;
    pub use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
    pub use std::iter::*;
}

aoc_runner_derive::aoc_lib! { year = 2024 }
