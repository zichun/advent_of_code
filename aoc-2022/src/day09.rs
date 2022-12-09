use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Dir {
    Up, Down, Left, Right
}

struct Instruction(Dir, usize);

struct Knot {
    knots: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
}

fn move_dir(coord: (i32, i32), dir: Dir) -> (i32, i32) {
    match dir {
        Dir::Up => (coord.0, coord.1 + 1),
        Dir::Down => (coord.0, coord.1 - 1),
        Dir::Left => (coord.0 - 1, coord.1),
        Dir::Right => (coord.0 + 1, coord.1),
    }
}
impl Knot {
    fn new(knots_cnt: usize) -> Self {
        assert!(knots_cnt > 0);
        let knots = (0..knots_cnt).map(|_| (0, 0)).collect();
        Knot {
            knots,
            visited: vec![(0, 0)].into_iter().collect(),
        }
    }
    fn move_knot(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.1 {
            self.knots[0] = move_dir(self.knots[0], instruction.0);
            self.resolve_tails();
        }
    }

    fn resolve_tails(&mut self) {
        for i in 1..self.knots.len() {
            Self::resolve_tail(self.knots[i - 1].clone(), &mut self.knots[i]);
        }
        self.visited.insert(*self.knots.last().unwrap());
    }

    fn resolve_tail(head: (i32, i32), tail: &mut (i32, i32)) {
        fn resolve(head0: &i32, tail0: &mut i32, head1: &i32, tail1: &mut i32) {
            let diff0 = (*head0 - *tail0).abs();
            let diff1 = (*head1 - *tail1).abs();
            if diff0 == 2 {
                *tail0 += (*head0 - *tail0) / 2;
                if diff1 >= 1 {
                    *tail1 += if (*head1 - *tail1).is_positive() { 1 } else { -1 };
                }
            }
        }
        resolve(&head.0, &mut tail.0, &head.1, &mut tail.1);
        resolve(&head.1, &mut tail.1, &head.0, &mut tail.0);
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| {
        let mut iter = line.split(" ");
        let dir = iter.next().unwrap();
        let size = iter.next().unwrap().parse::<usize>().unwrap();
        match dir {
            "U" => Instruction(Dir::Up, size),
            "D" => Instruction(Dir::Down, size),
            "L" => Instruction(Dir::Left, size),
            "R" => Instruction(Dir::Right, size),
            _ => unimplemented!()
        }
    }).collect()
}

pub fn part1(input: &str) -> usize {
    let instructions = parse(input);
    let mut knot = Knot::new(2);
    instructions.iter().for_each(|int| knot.move_knot(int));
    knot.visited.len()
}

pub fn part2(input: &str) -> usize {
    let instructions = parse(input);
    let mut knot = Knot::new(10);
    instructions.iter().for_each(|int| knot.move_knot(int));
    knot.visited.len()
}

#[test]
fn test() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(part1(input), 13);
    assert_eq!(part2(input), 1);

    let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    assert_eq!(part2(input), 36);
}
