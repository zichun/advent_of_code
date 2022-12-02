use std::collections::{ HashSet, HashMap };

struct Path(Vec<(i8, i8, i8)>);

impl Path {
    fn parse(line: &str) -> Self {
        Path(
            line.chars()
                .fold((Vec::new(), '_'),
                      |(mut acc, pre), el| {
                          match (pre, el) {
                              ('s', 'e') => {
                                  acc.push((1, 0, -1));
                                  (acc, '_')
                              },
                              ('s', 'w') => {
                                  acc.push((0, -1, -1));
                                  (acc, '_')
                              },
                              ('n', 'e') => {
                                  acc.push((0, 1, 1));
                                  (acc, '_')
                              }
                              ('n', 'w') => {
                                  acc.push((-1, 0, 1));
                                  (acc, '_')
                              },
                              ('_', 'e') => {
                                  acc.push((1, 1, 0));
                                  (acc, '_')
                              },
                              ('_', 'w') => {
                                  acc.push((-1, -1, 0));
                                  (acc, '_')
                              },
                              ('_', c) => (acc, c),
                              (_, _) => panic!("Invalid input")
                          }
                      }).0)
    }
    fn walk(&self, start: (i32, i32, i32)) -> (i32, i32, i32) {
        self.0.iter().fold(start, |acc, el| {
            (acc.0 + el.0 as i32, acc.1 + el.1 as i32, acc.2 + el.2 as i32)
        })
    }
}

pub fn day24_1(input: &str) -> usize {
    let paths = input.lines()
        .map(|line| Path::parse(line))
        .collect::<Vec<_>>();

    let mut blacks = HashSet::new();
    for p in paths {
        let current = p.walk((0, 0, 0));
        if blacks.contains(&current) {
            blacks.remove(&current);
        } else {
            blacks.insert(current);
        }
    }
    blacks.len()
}

pub fn day24_2(input: &str) -> usize {
    let paths = input.lines()
        .map(|line| Path::parse(line))
        .collect::<Vec<_>>();

    let mut blacks = HashSet::new();
    for p in paths {
        let current = p.walk((0, 0, 0));
        if blacks.contains(&current) {
            blacks.remove(&current);
        } else {
            blacks.insert(current);
        }
    }

    let adj = vec![(-1, 0, 1), (0, 1, 1), (-1, -1, 0),
                   (1, 1, 0), (0, -1, -1), (1, 0, -1)];
    for _ in 0..100 {
        let mut adj_cnt = HashMap::new();
        blacks.iter().for_each(|black| {
            adj.iter().for_each(|adj| {
                *adj_cnt.entry((black.0 + adj.0, black.1 + adj.1, black.2 + adj.2)).or_insert(0) += 1;
            });
        });
        blacks = adj_cnt.iter().filter_map(|(&coord, &cnt)| {
            let is_black = blacks.contains(&coord);
            match (is_black, cnt) {
                (false, 2) | (true, 1) | (true, 2) => Some(coord),
                _ => None
            }
        }).collect::<HashSet<_>>();
    }

    blacks.len()
}

#[test]
fn test() {
    let path = Path::parse("nwwswee");
    assert_eq!(path.walk((1, 1, 1)), (1, 1, 1));

    let path = Path::parse("swwsw");
    assert_eq!(path.walk((0, 0, 0)), (-1, -3, -2));
    let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
    assert_eq!(day24_1(input), 10);
    assert_eq!(day24_2(input), 2208);
}
