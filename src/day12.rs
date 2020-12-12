#[derive(PartialEq, Clone, Copy)]
enum Dir
{
    North,
    South,
    East,
    West
}

struct Ferry
{
    northSouth: i32,
    eastWest: i32,
    direction: Dir
}

impl Ferry
{
    fn new() -> Self
    {
        Ferry {
            northSouth: 0,
            eastWest: 0,
            direction: Dir::East
        }
    }

    fn process(&mut self, input: &str)
    {
        let mut c = input.chars();
        let cmd = c.next().unwrap();
        let qty = c.collect::<String>().parse::<i32>().unwrap();

        match cmd
        {
            'N' => self.move_ferry(Dir::North, qty),
            'S' => self.move_ferry(Dir::South, qty),
            'E' => self.move_ferry(Dir::East, qty),
            'W' => self.move_ferry(Dir::West, qty),
            'F' => self.move_ferry(self.direction, qty),
            'R' => self.turn_right(qty),
            'L' => self.turn_left(qty),
            _ => panic!("Unrecognized command")
        }
    }

    fn move_ferry(&mut self, dir: Dir, units: i32)
    {
        match dir
        {
            Dir::North => self.northSouth += units,
            Dir::South => self.northSouth -= units,
            Dir::East => self.eastWest += units,
            Dir::West => self.eastWest -= units
        }
    }

    fn turn_right(&mut self, units: i32)
    {
        let dir = vec![Dir::North, Dir::East, Dir::South, Dir::West];
        let mut x = dir.iter().position(|x| *x == self.direction).unwrap();
        x = (x + ((units / 90) as usize)) % 4;
        self.direction = dir[x];
    }

    fn turn_left(&mut self, units: i32)
    {
        self.turn_right(360 - units);
    }
}

struct FerryWithWaypoint
{
    northSouth: i32,
    eastWest: i32,
    waypointNorthSouth: i32, // relative to Ferry
    waypointEastWest: i32
}

impl FerryWithWaypoint
{
    fn new() -> Self {
        FerryWithWaypoint {
            northSouth: 0,
            eastWest: 0,
            waypointNorthSouth: 1,
            waypointEastWest: 10
        }
    }

    fn process(&mut self, input: &str)
    {
        let mut c = input.chars();
        let cmd = c.next().unwrap();
        let qty = c.collect::<String>().parse::<i32>().unwrap();

        match cmd {
            'N' => self.move_waypoint(Dir::North, qty),
            'S' => self.move_waypoint(Dir::South, qty),
            'E' => self.move_waypoint(Dir::East, qty),
            'W' => self.move_waypoint(Dir::West, qty),
            'F' => self.move_ferry(qty),
            'R' => self.turn_right(qty),
            'L' => self.turn_right(360 - qty),
            _ => panic!("Unrecognized command")
        }
    }

    fn move_waypoint(&mut self, dir: Dir, units: i32)
    {
        match dir
        {
            Dir::North => self.waypointNorthSouth += units,
            Dir::South => self.waypointNorthSouth -= units,
            Dir::East => self.waypointEastWest += units,
            Dir::West => self.waypointEastWest -= units
        }
    }

    fn move_ferry(&mut self, units: i32)
    {
        self.northSouth += units * self.waypointNorthSouth;
        self.eastWest += units * self.waypointEastWest;
    }

    fn turn_right(&mut self, units: i32) {
        for _ in 0..units / 90
        {
            let ew = self.waypointEastWest;
            self.waypointEastWest = self.waypointNorthSouth;
            self.waypointNorthSouth = -ew;
        }
    }
}

pub fn day12_1(input: &str) -> i32
{
    let mut ferry = Ferry::new();
    input.lines().for_each(|line| ferry.process(line));
    ferry.northSouth.abs() + ferry.eastWest.abs()
}

pub fn day12_2(input: &str) -> i32
{
    let mut ferry = FerryWithWaypoint::new();
    input.lines().for_each(|line| ferry.process(line));
    ferry.northSouth.abs() + ferry.eastWest.abs()
}

#[test]
fn test_day12_1()
{
    let input = "F10
N3
F7
R90
F11";
    assert_eq!(day12_1(input), 25);
}

#[test]
fn test_day12_2()
{
    let input = "F10
N3
F7
R90
F11";
    assert_eq!(day12_2(input), 286);
}
