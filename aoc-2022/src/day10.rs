enum Instruction {
    Addx(i32),
    Noop
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().flat_map(|line| {
        let mut iter = line.split(" ");
        match iter.next().unwrap() {
            "noop" => vec![Instruction::Noop],
            "addx" => vec![Instruction::Noop,
                           Instruction::Addx(iter.next().unwrap().parse().unwrap())],
            _ => unimplemented!()
        }
    }).collect()
}

pub fn part1(input: &str) -> i32 {
    let instructions = parse(input);
    let interesting = vec![20, 60, 100, 140, 180, 220];
    instructions.iter().enumerate()
        .fold((1, 0), |(mut x, mut sum), (cycle, int)| {
            if interesting.contains(&(cycle + 1)) {
                sum += x * (cycle + 1) as i32;
            }
            match int {
                Instruction::Addx(v) => x += *v,
                Instruction::Noop => (),
            };
            (x, sum)
        }).1

}

struct CRT {
    width: usize,
    height: usize,
    cur: (usize, usize),
    sprite: i32,
    screen: Vec<Vec<bool>>
}

impl CRT {
    fn new(width: usize, height: usize) -> Self {
        CRT {
            width,
            height,
            cur: (0, 0),
            sprite: 0,
            screen: vec![vec![false; width]; height],
        }
    }
    fn next(&mut self, new_sprite: i32) {
        if self.cur.0 >= self.height {
            return;
        }
        if self.cur.1 as i32 - self.sprite >= 0 && self.cur.1 as i32 - self.sprite < 3 {
            self.screen[self.cur.0][self.cur.1] = true;
        }
        self.sprite = new_sprite;
        self.cur.1 += 1;
        if self.cur.1 >= self.width {
            self.cur.1 = 0;
            self.cur.0 += 1;
        }
    }
}

pub fn part2(input: &str) {
    let instructions = parse(input);
    let mut crt = CRT::new(40, 6);

    instructions.iter()
        .fold(0, |mut x, int| {
            match int {
                Instruction::Addx(v) => x += *v,
                Instruction::Noop => (),
            };
            crt.next(x);
            x
        });

    for r in 0..6 {
        for c in 0..40 {
            print!("{}", if crt.screen[r][c] { "#" } else { "." });
        }
        println!("");
    }
}

#[test]
fn test() {
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    assert_eq!(part1(input), 13140);
    part2(input);
}
