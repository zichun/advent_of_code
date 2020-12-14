pub fn day13_1(input: &str) -> u32
{
    let mut inp = input.lines();
    let leave_time = inp.next().unwrap().parse::<u32>().unwrap();
    let min = inp.next().unwrap()
        .split(',')
        .map(|x| if x == "x" {
            0
        } else {
            x.parse::<u32>().unwrap()
        })
        .filter(|x| *x != 0)
        .map(|x| {
            (x, x * ((leave_time + x - 1) / x) - leave_time)
        })
        .min_by(|left, right| {
            left.1.cmp(&right.1)
        }).unwrap();

    min.0 * min.1
}

pub fn day13_2(input: &str) -> i64
{
    let mut inp = input.lines();
    let _ = inp.next();
    let inp: Vec<u32> = inp.next().unwrap()
        .split(',')
        .map(|x| if x == "x" {
            0
        } else {
            x.parse::<u32>().unwrap()
        })
        .collect();

    let mut n = Vec::new();
    let mut a = Vec::new();
    for i in 0..inp.len()
    {
        if inp[i] != 0 {
            a.push(inp[i] as i64 - i as i64);
            n.push(inp[i] as i64);
        }
    }
    gauss(&a, &n)
}

fn extended_euclid(x: i64, y: i64) -> (i64, i64, i64) {
    if x == 0
    {
        (y, 0, 1)
    }
    else
    {
        let (g, x1, y1) = extended_euclid(y % x, x);
        (g, y1 - (y / x) * x1, x1)
    }
}

fn invmod(a: i64, m: i64) -> i64 {
    let (g, x, _) = extended_euclid(a, m);
    if g != 1 {
        panic!("inv mod does not exist");
    }
    (x % m + m) % m
}

fn gauss(a: &[i64], n: &[i64]) -> i64 {
    let mut result = 0;
    let N = n.iter().product::<i64>();

    for i in 0..n.len() {
        let b = N / n[i];
        result += a[i] * b * invmod(b, n[i]);
    }

    result % N
}

#[test]
fn test_day13_1()
{
    let input = "939
7,13,x,x,59,x,31,19";
    assert_eq!(day13_1(input), 295);
}

#[test]
fn test_day13_2()
{
    let input = "939
7,13,x,x,59,x,31,19";
    assert_eq!(day13_2(input), 1068781);

    let input = "939
17,x,13,19";
    assert_eq!(day13_2(input), 3417);

    let input = "939
67,7,59,61";
    assert_eq!(day13_2(input), 754018);

    let input = "939
1789,37,47,1889";
    assert_eq!(day13_2(input), 1202161486);
}
