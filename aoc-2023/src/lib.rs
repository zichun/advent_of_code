mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub mod prelude {
    pub use aoc_runner_derive::{aoc_generator, aoc};
    pub use std::collections::{HashMap, HashSet, VecDeque};
    pub use std::collections::hash_map::Entry;
    pub use itertools::{Itertools, iproduct};
    pub use regex::{Regex, Captures};

    pub trait TokenReader {
        fn next_token<T: std::str::FromStr>(&mut self) -> T
        where <T as std::str::FromStr>::Err: std::fmt::Debug;
    }
    impl<'a, I: Iterator<Item=&'a str>> TokenReader for I {
        fn next_token<T: std::str::FromStr>(&mut self) -> T
        where <T as std::str::FromStr>::Err: std::fmt::Debug {
            self.next().unwrap().parse::<T>().unwrap()
        }
    }
    pub trait InputHelper<'a> where Self: Sized {
        fn extract_tokens<T: std::str::FromStr>(&self) -> Box<dyn Iterator<Item = T> + 'a>;
        fn parse_tokens<T: std::str::FromStr>(&self) -> Box<dyn Iterator<Item = T> + 'a>
        where <T as std::str::FromStr>::Err: std::fmt::Debug;
    }
    impl<'a> InputHelper<'a> for &'a str {
        fn extract_tokens<T: std::str::FromStr>(&self) -> Box<dyn Iterator<Item = T> + 'a> {
            Box::new(self.split_whitespace().filter_map(|t| t.parse::<T>().ok()))
        }

        fn parse_tokens<T: std::str::FromStr>(&self) -> Box<dyn Iterator<Item = T> + 'a>
        where <T as std::str::FromStr>::Err: std::fmt::Debug
        {
            Box::new(self.split_whitespace().map(|t| t.parse::<T>().unwrap()))
        }
    }

    pub fn bsearch<I, F>(mut left: I, mut right: I, mut test: F) -> I
    where I: num::Integer + Copy + From<u8>, F: FnMut(I) -> bool
    {
        while left < right {
            let mid = (left + right) / I::from(2);
            if test(mid) {
                left = mid + I::one();
            } else {
                right = mid - I::one();
            }
        }
        left
    }

    pub fn tsearch<I, F>(mut left: I, mut right: I, mut cmp: F) -> I
    where I: num::Integer + Copy + From<u8>, F: FnMut(I, I) -> bool
    {
        while right > left {
            let left_split = left + (right - left) / I::from(3);
            let right_split = right - (right - left) / I::from(3);
            if cmp(left_split, right_split) {
                left = left_split + I::one();
            } else {
                right = right_split - I::one();
            }
        }
        left
    }
}

aoc_runner_derive::aoc_lib!{ year = 2023 }
