pub fn day10_1(input: &str) -> u32
{
    let mut inp: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    inp.sort();

    let (_, one_diff, three_diff) = inp.iter().skip(1).fold((inp[0], 1, 1), |(prev, one_diff, three_diff), &el| {
        assert_eq!(el > prev && el <= prev + 3, true);
        if el == prev + 1 {
            (el, one_diff + 1, three_diff)
        } else if el == prev + 3 {
            (el, one_diff, three_diff + 1)
        } else {
            (el, one_diff, three_diff)
        }
    });
    one_diff * three_diff
}

pub fn day10_2(input: &str) -> u64
{
    let mut inp: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    inp.push(0);
    inp.sort();

    fn part2(inp: &[u32], memo: &mut Vec<u64>, ind: usize) -> u64
    {
        if ind + 1 >= inp.len() {
            return 1;
        }

        if memo[ind] != 0 {
            return memo[ind];
        }

        let mut ans = 0;
        for i in ind+1..inp.len() {
            if inp[i] > inp[ind] + 3 {
                break;
            }
            ans += part2(inp, memo, i);
        }
        memo[ind] = ans;
        ans
    }

    let mut memo = Vec::new();
    memo.resize_with(inp.len(), || 0);
    part2(&inp, &mut memo, 0)
}

#[test]
fn test_day10()
{
    let inp = "16
10
15
5
1
11
7
19
6
12
4";
    assert_eq!(day10_1(inp), 35);
    assert_eq!(day10_2(inp), 8);

    let inp = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    assert_eq!(day10_1(inp), 220);
    assert_eq!(day10_2(inp), 19208);
}
