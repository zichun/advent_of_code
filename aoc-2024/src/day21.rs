use std::sync::{LazyLock, Mutex};
use once_cell::sync::Lazy;
use crate::prelude::*;

type LazyKeyMap = Lazy<Mutex<Option<HashMap<char, (usize, usize)>>>>;
trait KeyPadRep {
    fn get_str_rep() -> &'static str;
    fn get_key_map()-> &'static LazyKeyMap;
}
struct KeyPad<T> {
    _t: std::marker::PhantomData<T>
}
impl<T: KeyPadRep> KeyPad<T> {
    fn get_key_pos(key: char) -> (usize, usize) {
        *T::get_key_map().lock().unwrap().as_ref().unwrap().get(&key).unwrap()
    }
    fn get_key(r: usize, c: usize) -> char {
        T::get_str_rep().lines().skip(r).next().unwrap().chars().skip(c).next().unwrap()
    }
    fn path(from: char, to: char) -> Vec<char> {
        let (sr, sc) = Self::get_key_pos(from);
        let (r, c) = Self::get_key_pos(to);
        let mut vert: Vec<char> = if sr > r {
            repeat_n('^', sr - r).collect()
        } else {
            repeat_n('v', r - sr).collect()
        };
        let mut hor: Vec<char> = if sc > c {
            repeat_n('<', sc - c).collect()
        } else {
            repeat_n('>', c - sc).collect()
        };
        if (c > sc && Self::get_key(r, sc) != 'X') || Self::get_key(sr, c) == 'X' {
            vert.append(&mut hor);
            vert.push('A');
            vert
        } else {
            hor.append(&mut vert);
            hor.push('A');
            hor
        }
    }
}
fn generate_key_map(static_map: &mut LazyKeyMap, s: &str) {
    let mut map = static_map.lock().unwrap();
    if map.is_none() {
        let new_map = s.lines().enumerate().flat_map(|(r, row)| {
            row.chars().enumerate().map(move |(c, ch)| (ch, (r, c)))
        }).collect::<HashMap<char, (usize, usize)>>();
        *map = Some(new_map);
    }
}
struct DirPad {}
impl KeyPadRep for DirPad {
    fn get_str_rep() -> &'static str {
        "X^A
<v>"
    }
    fn get_key_map() -> &'static Lazy<Mutex<Option<HashMap<char, (usize, usize)>>>> {
        static mut MAP: LazyKeyMap = Lazy::new(|| Mutex::new(None));
        unsafe {
            generate_key_map(&mut MAP, Self::get_str_rep());
            &MAP
        }
    }
}
struct NumPad {}
impl KeyPadRep for NumPad {
    fn get_str_rep() -> &'static str {
        "789
456
123
X0A"
    }
    fn get_key_map() -> &'static Lazy<Mutex<Option<HashMap<char, (usize, usize)>>>> {
        static mut MAP: LazyKeyMap = Lazy::new(|| Mutex::new(None));
        unsafe {
            generate_key_map(&mut MAP, Self::get_str_rep());
            &MAP
        }
    }
}

fn solve<const D: usize>(code: &str) -> usize {
    type MemoType = HashMap<(char, char, usize), usize>;
    static mut MEMO: LazyLock<Mutex<MemoType>> = LazyLock::new(|| Mutex::new(HashMap::new()));

    fn recur<const D: usize>(dir_from: char, dir_to: char, depth: usize, memo: &mut MemoType) -> usize {
        if depth == 0 {
            1
        } else if memo.contains_key(&(dir_from, dir_to, depth)) {
            memo[&(dir_from, dir_to, depth)]
        } else {
            let dirs = KeyPad::<DirPad>::path(dir_from, dir_to);
            let mut tr = 0;
            dirs.clone().into_iter().fold('A', |from, to| {
                tr += recur::<D>(from, to, depth - 1, memo);
                to
            });
            memo.insert((dir_from, dir_to, depth), tr);
            tr
        }
    }

    let mut memo = unsafe { MEMO.lock().unwrap() };
    let mut tr = 0;
    code.chars().fold('A', |from, to| {
        let dirs = KeyPad::<NumPad>::path(from, to);
        dirs.into_iter().fold('A', |from, to| {
            tr += recur::<D>(from, to, D, &mut memo);
            to
        });
        to
    });
    code[0..code.len() - 1].to_string().parse::<usize>().unwrap() * tr
}

#[aoc(day21, part1)]
fn part1(inp: &str) -> usize {
    inp.lines().map(solve::<2>).sum()
}

#[aoc(day21, part2)]
fn part2(inp: &str) -> usize {
    inp.lines().map(solve::<25>).sum()

}
