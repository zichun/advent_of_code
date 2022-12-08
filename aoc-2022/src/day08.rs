#[derive(Default, Debug)]
struct Tree {
    height: u32,
    visible: bool,
    scenic: Vec<usize>,
}

struct Field(Vec<Vec<Tree>>);

impl Field {
    fn total_visible(&self) -> usize {
        self.0.iter()
            .map(|r| r.iter().filter(|t| t.visible).count())
            .sum::<usize>()
    }
    fn max_scenic(&self) -> usize {
        self.0.iter()
            .map(|r| r.iter().map(|t|
                                  t.scenic.iter().product::<usize>()).max().unwrap() )
            .max().unwrap()

    }
}

fn parse(input: &str) -> Field {
    let f = input.lines().map(|l|
                              l.chars().map(|c| Tree {
                                  height: c as u32 - '0' as u32,
                                  ..Tree::default()
                              }).collect()
    ).collect();
    Field(f)
}

fn transpose(r: usize, c: usize, board_size: usize, dir: usize) -> (usize, usize) {
    match dir {
        0 => (r, c),
        1 => (r, board_size - 1 - c),
        2 => (c, r),
        3 => (board_size - 1 - c, board_size - 1 - r),
        _ => unimplemented!()
    }
}

fn diff(r: usize, c: usize, rr: usize, cc: usize) -> usize {
    fn abs_diff(a: usize, b: usize) -> usize {
        if a > b { a - b } else { b - a }
    }
    abs_diff(r, rr) + abs_diff(c, cc)
}

pub fn part2(input: &str) -> usize {
    let mut f = parse(input);
    let board_size = f.0.len();
    for dir in 0..4 {
        for r in 0..board_size {
            let mut stack: Vec<(u32, usize, usize, usize)> = Vec::new();
            for c in 0..board_size {
                let (rr, cc) = transpose(r, c, board_size, dir);
                let el = f.0[rr][cc].height;
                while !stack.is_empty() && el >= stack.last().unwrap().0 {
                    let (_, r, c, _) = stack.pop().unwrap();
                    f.0[r][c].scenic.push(diff(r, c, rr, cc));
                }
                stack.push((el, rr, cc, c));
            }
            stack.iter()
                .for_each(|(_, r, c, oc)|
                          f.0[*r][*c].scenic.push(board_size - oc - 1));
        }
    }
    f.max_scenic()
}

pub fn part1(input: &str) -> usize {
    let mut f = parse(input);
    let board_size = f.0.len();
    for dir in 0..4 {
        for r in 0..board_size {
            let mut visible: Vec<(u32, usize, usize)> = Vec::new();
            for c in 0..board_size {
                let (rr, cc) = transpose(r, c, board_size, dir);
                let el = f.0[rr][cc].height;
                if visible.is_empty() || el > visible.last().unwrap().0 {
                    visible.push((el, rr, cc));
                }
            }
            visible.iter().for_each(|(_, r, c)| f.0[*r][*c].visible = true);
        }
    }
    f.total_visible()
}

#[test]
fn test() {
    let input = "30373
25512
65332
33549
35390";
    assert_eq!(part1(input), 21);
    assert_eq!(part2(input), 8);
}
