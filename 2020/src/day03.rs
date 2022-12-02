struct Map
{
    map: Vec<Vec<bool>>
}

impl Map
{
    fn parse(input: &str) -> Self
    {
        Map {
            map: input.lines().map(
                |line|
                {
                    let is_tree: Vec<bool> = line.chars().map(|c| c == '#').collect();
                    is_tree
                }
            ).collect()
        }
    }

    fn is_tree(&self, row: usize, col: usize) -> bool
    {
        self.map[row][col % self.map[0].len()]
    }

    fn rows(&self) -> usize
    {
        self.map.len()
    }

    fn slope(&self, right: usize, down: usize) -> u32 {
        let mut row = 0;
        let mut col = 0;
        let mut cnt = 0;
        while row + down < self.rows() {
            row += down;
            col += right;
            cnt += if self.is_tree(row, col) { 1 } else { 0 };
        }
        cnt
    }
}

pub fn day03_1(input: &str) -> u32
{
    let map = Map::parse(input);
    map.slope(3, 1)
}

pub fn day03_2(input: &str) -> u64
{
    let map = Map::parse(input);

    vec![(1, 1),
         (3, 1),
         (5, 1),
         (7, 1),
         (1, 2)]
        .iter()
        .map(|x| {
            map.slope(x.0, x.1) as u64
        })
        .product()
}

#[test]
fn test_day03_1() {
    let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"#;
    assert_eq!(day03_1(&input), 7);
}

#[test]
fn test_day03_2() {
    let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"#;
    assert_eq!(day03_2(&input), 336);
}
