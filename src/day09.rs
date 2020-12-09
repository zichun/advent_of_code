use std::collections::VecDeque;

fn part1(inp: &[u64], window: usize) -> u64
{
    fn valid(vec: &VecDeque<u64>, val: u64) -> bool
    {
        for i in 0..vec.len()
        {
            for j in i+1..vec.len()
            {
                if vec[i] + vec[j] == val {
                    return true;
                }
            }
        }
        false
    }
    let mut vec = VecDeque::new();

    for i in 0..window {
        vec.push_back(inp[i]);
    }
    for i in window..inp.len() - 1 {
        if !valid(&vec, inp[i]) {
            return inp[i];
        }
        vec.push_back(inp[i]);
        vec.pop_front();
    }
    panic!("Cannot find solution");
}

pub fn day09_1(inp: &str, window: usize) -> u64
{
    let inp: Vec<u64> = inp.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    part1(&inp, window)
}

pub fn day09_2(inp: &str, window: usize) -> u64
{
    let inp: Vec<u64> = inp.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    let mut acc = Vec::new();
    let target = part1(&inp, window);

    acc.push(inp[0]);
    for i in 1..inp.len()
    {
        acc.push(acc[i - 1] + inp[i]);
    }

    for i in 0..inp.len()
    {
        for j in i + 1..inp.len()
        {
            let mut sum = acc[j];
            if i > 0 { sum -= acc[i - 1]; }
            if sum == target {
                return inp[i..j].iter().min().unwrap() + inp[i..j].iter().max().unwrap();
            }
        }
    }
    panic!("Cannot find solution");
}

#[test]
fn test_day09_part1() {
    let inp = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    assert_eq!(day09_1(&inp, 5), 127);
}
#[test]
fn test_day09_part2() {
    let inp = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    assert_eq!(day09_2(&inp, 5), 62);
}
