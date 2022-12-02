use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Node {
    r: usize,
    c: usize,
    cost: u32
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

pub fn p1(input: &str) -> u32 {
    let matrix = input.lines().map(|line|
                      line.chars().map(|c| (c as u32) - ('0' as u32)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut memo = vec![vec![false; rows]; cols];

    let mut heap = BinaryHeap::new();
    heap.push(Node { r: 0, c: 0, cost: 0});

    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    memo[0][0] = true;
    while let Some(n) = heap.pop() {
        if n.r == rows - 1 && n.c == cols - 1{
            return n.cost;
        }

        let mut candidate = Vec::new();
        for (rr, cc) in dir {
            let r = n.r as i32 + rr;
            let c = n.c as i32 + cc;
            if r < 0 || r >= rows as i32 ||
                c < 0 || c >= cols as i32
            {
                continue;
            }
            let r = r as usize;
            let c = c as usize;

            if memo[r][c]
            {
                continue;
            }

            candidate.push((r, c, matrix[r][c] + n.cost));
        }
        candidate.sort_by(|a, b| {
            a.2.cmp(&b.2)
        });
        if let Some((r, c, cost)) = candidate.first() {
            println!("{} {}: {} {}", *r, *c, *cost, matrix[*r][*c]);
            memo[*r][*c] = true;
            heap.push(Node { r: *r, c: *c, cost: *cost } );
        }
    }
    panic!("No path found??");
}

#[test]
fn test() {
    let input = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;
//    assert_eq!(p1(input), 40);

    let input = r#"111
991
111
199
111"#;
//    assert_eq!(p1(input), 10);

}
