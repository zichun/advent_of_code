use crate::prelude::*;

#[aoc(day8, part2)]
fn part2(inp: &str) -> usize {
    let g: Grid<char> = Grid::from_str(inp).unwrap();

    let mut ant = HashMap::new();
    g.iter().for_each(|((r, c), ch)| {
        if ch.is_ascii_alphanumeric() {
            ant.entry(*ch)
                .or_insert(Vec::new())
                .push((r as isize, c as isize));
        }
    });

    let (rr, cc) = g.dimensions();
    let mut nodes = HashSet::new();
    fn insert(nr: isize, nc: isize, rr: usize, cc: usize, nodes: &mut HashSet<(isize, isize)>) {
        if nr >= 0 && nc >= 0 && nr < rr as isize && nc < cc as isize {
            nodes.insert((nr, nc));
        }
    }
    ant.iter().for_each(|(_ch, coords)| {
        coords.iter().combinations(2).for_each(|v| {
            let ((r0, c0), (r1, c1)) = (v[0], v[1]);
            let (dr, dc) = (r1 - r0, c1 - c0);

            for i in 0.. {
                if g.contains(r1 + i * dr, c1 + i * dc) {
                    insert(r1 + i * dr, c1 + i * dc, rr, cc, &mut nodes);
                } else {
                    break;
                }
            }
            for i in 0.. {
                if g.contains(r0 - i * dr, c0 - i * dc) {
                    insert(r0 - i * dr, c0 - i * dc, rr, cc, &mut nodes);
                } else {
                    break;
                }
            }
        });
    });

    nodes.len()
}

#[aoc(day8, part1)]
fn part1(inp: &str) -> usize {
    let g: Grid<char> = Grid::from_str(inp).unwrap();

    let mut ant = HashMap::new();
    g.iter().for_each(|((r, c), ch)| {
        if ch.is_ascii_alphanumeric() {
            ant.entry(*ch)
                .or_insert(Vec::new())
                .push((r as isize, c as isize));
        }
    });

    let (rr, cc) = g.dimensions();
    let mut nodes = HashSet::new();
    fn insert(nr: isize, nc: isize, rr: usize, cc: usize, nodes: &mut HashSet<(isize, isize)>) {
        if nr >= 0 && nc >= 0 && nr < rr as isize && nc < cc as isize {
            nodes.insert((nr, nc));
        }
    }
    ant.iter().for_each(|(_ch, coords)| {
        coords.iter().combinations(2).for_each(|v| {
            let ((r0, c0), (r1, c1)) = (v[0], v[1]);
            let (dr, dc) = (r1 - r0, c1 - c0);

            insert(r1 + dr, c1 + dc, rr, cc, &mut nodes);
            insert(r0 - dr, c0 - dc, rr, cc, &mut nodes);
        });
    });

    nodes.len()
}
