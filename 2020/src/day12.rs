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
    north_south: i32,
    east_west: i32,
    direction: Dir
}

impl Ferry
{
    fn new() -> Self
    {
        Ferry {
            north_south: 0,
            east_west: 0,
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
            Dir::North => self.north_south += units,
            Dir::South => self.north_south -= units,
            Dir::East => self.east_west += units,
            Dir::West => self.east_west -= units
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
    north_south: i32,
    east_west: i32,
    waypoint_north_south: i32, // relative to Ferry
    waypoint_east_west: i32
}

impl FerryWithWaypoint
{
    fn new() -> Self {
        FerryWithWaypoint {
            north_south: 0,
            east_west: 0,
            waypoint_north_south: 1,
            waypoint_east_west: 10
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
            Dir::North => self.waypoint_north_south += units,
            Dir::South => self.waypoint_north_south -= units,
            Dir::East => self.waypoint_east_west += units,
            Dir::West => self.waypoint_east_west -= units
        }
    }

    fn move_ferry(&mut self, units: i32)
    {
        self.north_south += units * self.waypoint_north_south;
        self.east_west += units * self.waypoint_east_west;
    }

    fn turn_right(&mut self, units: i32) {
        for _ in 0..units / 90
        {
            let ew = self.waypoint_east_west;
            self.waypoint_east_west = self.waypoint_north_south;
            self.waypoint_north_south = -ew;
        }
    }
}

pub fn day12_1(input: &str) -> i32
{
    let mut ferry = Ferry::new();
    input.lines().for_each(|line| ferry.process(line));
    ferry.north_south.abs() + ferry.east_west.abs()
}

pub fn day12_2(input: &str) -> i32
{
    let mut ferry = FerryWithWaypoint::new();
    input.lines().for_each(|line| ferry.process(line));
    ferry.north_south.abs() + ferry.east_west.abs()
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
