struct Board {
    pos: [Option<(usize, usize)>; 100],
    marked: [[bool; 5]; 5],
}
impl Board {
    fn new() -> Self {
        Board {
            pos: [None; 100],
            marked: [[false; 5]; 5],
        }
    }

    fn play(&mut self, num: usize) -> bool {
        if let Some(pos) = self.pos[num] {
            self.marked[pos.0][pos.1] = true;
            self.check_win()
        } else {
            false
        }
    }

    fn check_win(&self) -> bool {
        for r in 0..5 {
            let mut win0 = true;
            let mut win1 = true;
            for c in 0..5 {
                win0 &= self.marked[r][c];
                win1 &= self.marked[c][r];
            }

            if win0 || win1 {
                return true;
            }
        }
        false
    }

    fn sum_unchecked(&self) -> u32 {
        let mut sum = 0;
        for n in 0..100 {
            if let Some((r, c)) = self.pos[n] {
                if !self.marked[r][c] {
                    sum += n as u32;
                }
            }
        }
        sum
    }

}

pub fn p2(input: &str) -> u32 {
    let mut input = input.lines();
    let draw = input.next().unwrap().split(",").map(|x| {
        x.parse::<usize>().unwrap()
    }).collect::<Vec<_>>();

    let mut boards = Vec::new();
    let mut won = Vec::new();

    while let Some(line) = input.next() {
        if line.trim().len() == 0 {
            continue;
        }

        let mut board = Board::new();

        let mut l = line;
        for row in 0..5 {
            l.split_ascii_whitespace().enumerate().for_each(|(col, num)| {
                let num = num.parse::<u32>().unwrap() as usize;
                board.pos[num] = Some((row, col));
            });

            if let Some(nl) = input.next() {
                l = nl;
            }
        }

        boards.push(board);
        won.push(false);
    }

    let mut win = 0;

    let n = boards.len();
    for d in draw.iter() {
        for i in 0..n {
            let b = &mut boards[i];
            if b.play(*d) && !won[i] {
                won[i] = true;
                win += 1;

                if win == n {
                    return (*d as u32) * b.sum_unchecked();
                }
            }
        }
    }
    panic!("No winner");

}
pub fn p1(input: &str) -> u32 {
    let mut input = input.lines();
    let draw = input.next().unwrap().split(",").map(|x| {
        x.parse::<usize>().unwrap()
    }).collect::<Vec<_>>();

    let mut boards = Vec::new();

    while let Some(line) = input.next() {
        if line.trim().len() == 0 {
            continue;
        }

        let mut board = Board::new();

        let mut l = line;
        for row in 0..5 {
            l.split_ascii_whitespace().enumerate().for_each(|(col, num)| {
                let num = num.parse::<u32>().unwrap() as usize;
                board.pos[num] = Some((row, col));
            });

            if let Some(nl) = input.next() {
                l = nl;
            }
        }

        boards.push(board);
    }

    for d in draw.iter() {
        for b in boards.iter_mut() {
            if b.play(*d) {
                return (*d as u32) * b.sum_unchecked();
            }
        }
    }
    panic!("No winner");}

#[test]
fn test() {
    let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

    assert_eq!(p1(input), 4512);
    assert_eq!(p2(input), 1924);
}
