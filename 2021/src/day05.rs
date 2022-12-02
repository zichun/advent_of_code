
fn get_coord(input: &str) -> (usize, usize) {
    let mut iter = input.split(",");
    (iter.next().unwrap().parse::<usize>().unwrap(),
     iter.next().unwrap().parse::<usize>().unwrap())
}

#[derive(Debug)]
struct Input {
    from: (usize, usize),
    to: (usize, usize)
}

fn parse_input(input: &str) -> Vec<Input> {
    input.lines().map(
        |l| {
            let mut iter = l.split(" ");
            let first = get_coord(iter.next().unwrap());
            let second = get_coord(iter.skip(1).next().unwrap());
            Input {
                from: first,
                to: second
            }
        }).collect::<Vec<_>>()
}

pub fn p2(input: &str) -> usize {
    let input = parse_input(input);
    let mut cnt = vec![[0 as usize; 1024]; 1024];

    input.iter().for_each(|inp| {
        let mut cur = inp.from;

        while cur != inp.to {
            cnt[cur.1][cur.0] += 1;
            if cur.0 < inp.to.0 {
                cur.0 += 1;
            } else if cur.0 > inp.to.0 {
                cur.0 -= 1;
            }
            if cur.1 < inp.to.1 {
                cur.1 += 1
            } else if cur.1 > inp.to.1 {
                cur.1 -= 1
            };
        }
        cnt[cur.1][cur.0] += 1;
    });

    let mut tr = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            if cnt[y][x] >= 2 {
                tr += 1;
            }
        }
    }
    tr
}

pub fn p1(input: &str) -> usize {
    let input = parse_input(input);
    let mut cnt = vec![[0 as usize; 1024]; 1024];

    input.iter().for_each(|inp| {
        if inp.from.0 == inp.to.0
        {
            let f = inp.from.1.min(inp.to.1);
            let t = inp.from.1.max(inp.to.1);
            for y in f..=t {
                cnt[y][inp.from.0] += 1;
            }
        }

        if inp.from.1 == inp.to.1 {
            let f = inp.from.0.min(inp.to.0);
            let t = inp.from.0.max(inp.to.0);
            for x in f..=t {
                cnt[inp.from.1][x] += 1;
            }
        }
    });

    let mut tr = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            if cnt[y][x] >= 2 {
                tr += 1;
            }
        }
    }
    tr
}


#[test]
fn test() {
    let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;
    assert_eq!(p1(input), 5);
    assert_eq!(p2(input), 12);
}
