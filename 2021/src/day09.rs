pub fn p1(input: &str) -> u32 {
    let inp = input.lines().map(
        |line| {
            line.chars().map(|c| (c as u32) - ('0' as u32)).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fn is_low(r: usize, c: usize, inp: &Vec<Vec<u32>>) -> bool {
        let mut high = 0;

        if r == 0 || inp[r][c] < inp[r-1][c] {
            high += 1;
        }
        if c == 0 || inp[r][c] < inp[r][c-1] {
            high += 1;
        }
        if r == inp.len() - 1 || inp[r][c] < inp[r+1][c] {
            high += 1;
        }
        if c == inp[0].len() - 1 || inp[r][c] < inp[r][c+1] {
            high += 1;
        }
        high == 4
    }

    let mut tr = 0;
    for r in 0..inp.len() {
        for c in 0..inp[0].len() {
            if is_low(r, c, &inp) {
                tr += inp[r][c] + 1;
            }
        }
    }
    tr
}

#[test]
fn test() {
    let input = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    assert_eq!(p1(input), 15);
}
