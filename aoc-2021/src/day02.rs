enum Direction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl Direction {
    fn from(inp: &str) -> Self {
        let mut split = inp.split_ascii_whitespace();
        let dir = split.next().unwrap();
        let mag = split.next().unwrap().parse::<usize>().unwrap();
        match dir {
            "forward" => Direction::Forward(mag),
            "down" => Direction::Down(mag),
            "up" => Direction::Up(mag),
            _ => panic!("invalid dir")
        }
    }
}

pub fn day02_1(input: &str) -> usize
{
    let directions = input.lines().map(|line| {
        Direction::from(line)
    }).collect::<Vec<_>>();

    let mut x = 0;
    let mut y = 0;
    directions.iter().for_each(|dir| {
        match dir {
            Direction::Forward(mag) => x += *mag,
            Direction::Down(mag) => y += *mag,
            Direction::Up(mag) => y -= *mag,
        }
    });

    x * y
}

pub fn day02_2(input: &str) -> usize
{
    let directions = input.lines().map(|line| {
        Direction::from(line)
    }).collect::<Vec<_>>();

    let mut aim = 0;
    let mut y = 0;
    let mut x = 0;

    directions.into_iter().for_each(|d| {
        match d {
            Direction::Forward(mag) => {
                x += mag;
                y += mag * aim;
            },
            Direction::Down(mag) => aim += mag,
            Direction::Up(mag) => aim -= mag,
        }
    });

    x * y
}


#[test]
fn test() {
    assert_eq!(day02_1(r#"forward 5
down 5
forward 8
up 3
down 8
forward 2
"#), 150);

    assert_eq!(day02_2(r#"forward 5
down 5
forward 8
up 3
down 8
forward 2
"#), 900);
}
