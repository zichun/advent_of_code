mod day1;

pub mod prelude {
    pub use aoc_helper::prelude::*;
    pub use aoc_runner_derive::{aoc, aoc_generator};
    pub use itertools::{iproduct, Itertools};
    pub use regex::{Captures, Regex};
    pub use std::iter::*;
    pub use std::collections::hash_map::Entry;
    pub use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
}

aoc_runner_derive::aoc_lib! { year = 2024 }
