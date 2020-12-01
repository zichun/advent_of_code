use std::collections::HashSet;

const TARGET: u32 = 2020;

pub fn day01_1(input: &Vec<u32>) -> u32
{
    let mut hash = HashSet::new();

    input.iter().for_each(
        |x| { hash.insert(*x); }
    );

    let mut ans = None;
    input.iter().for_each(
        |x|
        if hash.contains(&((TARGET - x) as u32))
        {
            ans = Some(x * (TARGET - x));
        }
    );

    ans.unwrap()
}

pub fn day01_2(input: &Vec<u32>) -> u32
{
    let mut hash = HashSet::new();

    input.iter().for_each(
        |x| { hash.insert(*x); }
    );

    let mut ans = None;

    for i in 0..input.len()
    {
        for j in 1..input.len()
        {
            if input[i] + input[j] > TARGET
            {
                continue;
            }

            if hash.contains(&(TARGET - input[i] - input[j]))
            {
                ans = Some(input[i] * input[j] * (TARGET - input[i] - input[j]));
            }
        }
    }

    ans.unwrap()
}

#[test]
fn test_day01() {
    assert_eq!(day01_1(&vec![1721,979,366,299,675,1456]), 514579);
    assert_eq!(day01_2(&vec![1721,979,366,299,675,1456]), 241861950);
}
