use crate::prelude::*;

#[aoc(day9, part1)]
fn part1(inp: &str) -> usize {
    let mut files = VecDeque::new();
    let mut gaps = Vec::new();
    inp.chars().enumerate().for_each(|(ind, el)| {
        let digit = el.to_digit(10).unwrap() as usize;
        if ind % 2 == 0 {
            files.push_back((ind / 2, digit));
        } else {
            gaps.push(digit);
        }
    });

    let tot = files.iter().map(|(_, cnt)| *cnt).sum();

    let mut ind = 0;
    let mut cs = 0;
    'out: for i in 0..files.len() {
        for _ in 0..files[i].1 {
            cs += ind * files[i].0;
            ind += 1;
        }

        for _ in 0..gaps[i] {
            if ind < tot {
                let file = files.get_mut(files.len() - 1).unwrap();
                cs += ind * file.0;
                file.1 -= 1;
                if file.1 == 0 {
                    files.pop_back();
                }
                ind += 1;
            } else {
                break 'out;
            }
        }
    }

    cs
}

#[derive(Debug)]
enum Space {
    File(usize, usize, usize), // file_ind, len, left_ind
    Gap(usize, usize),         //len, left-ind
}

#[aoc(day9, part2)]
fn part2(inp: &str) -> usize {
    let mut files = Vec::new();
    let mut left_ind = 0;
    inp.chars().enumerate().for_each(|(ind, el)| {
        let digit = el.to_digit(10).unwrap() as usize;
        if ind % 2 == 0 {
            files.push(Space::File(ind / 2, digit, left_ind));
        } else {
            files.push(Space::Gap(digit, left_ind));
        }
        left_ind += digit;
    });

    let mut cs = 0;
    for i in (0..files.len()).rev() {
        if let Space::File(file_ind, file_len, left_ind) = files[i] {
            let mut found = false;
            for j in 0..i {
                if let Space::Gap(gap, left_ind) = files[j] {
                    if file_len <= gap {
                        found = true;
                        for k in left_ind..left_ind + file_len {
                            cs += file_ind * k;
                        }
                        files[j] = Space::Gap(gap - file_len, left_ind + file_len);
                        break;
                    }
                }
            }
            if !found {
                for k in left_ind..left_ind + file_len {
                    cs += file_ind * k;
                }
            }
        }
    }

    cs
}
