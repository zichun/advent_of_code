use itertools::Itertools;

struct Hanoi(Vec<Vec<char>>);

struct Instruction {
    move_cnt: usize,
    from: usize,
    to: usize,
}

fn parse(input: &str) -> (Hanoi, Vec<Instruction>) {
    let instructions = Vec::new();
    let hanoi = Vec::new();

    let mut iter = input.split("\n\n");
    let hanoi_text = iter.next().unwrap();
    let instructions_text = iter.next().unwrap();

    hanoi_text
        .lines().
        filter_map(|l| {
        });

    (Hanoi(hanoi), instructions)
}

pub fn part1(input: &str) -> String {
    let mut first_parse = true;
    let mut stacks = Vec::new();

    input.lines().for_each(|l| {
        if first_parse {
            let len = l.len();
            let mut i = 1;
            let mut ind = 0;

            while i < len {
                let c = l.as_bytes()[i] as char;
                if c.is_ascii_alphabetic() {
                    if ind >= stacks.len() {
                        stacks.push(Vec::new())
                    }
                    stacks[ind].push(c);
                } else {
                    first_parse = false;
                    break;
                }
                i += 4;
                ind += 1;
            }
        } else {
            if l.trim().len() == 0 {
                return;
            }
            // move 5 from 12 to 17
        }
    });
}

pub fn part2(input: &str) -> u32 {
    0
}

#[test]
fn test() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(part1(input), "CMZ".to_owned());
    assert_eq!(part2(input), 0);
}
