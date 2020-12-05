struct Seat
{
    row: u16,
    col: u16
}

impl Seat
{
    fn from_boarding_pass(code: &str) -> Self
    {
        assert_eq!(code.len(), 10);

        let fold_fn = |acc, el|
                  (acc << 1) | match el {
                      'F' | 'L' => 0 as u16,
                      'B' | 'R' => 1 as u16,
                      _ => panic!("Unsupported")
                  };

        Seat {
            row: code.chars().take(7).fold(0, fold_fn),
            col: code.chars().skip(7).take(3).fold(0, fold_fn)
        }
    }
    fn seat_id(&self) -> u16
    {
        self.row * 8 + self.col
    }
}

pub fn day05_1(input: &str) -> u16
{
    input
        .lines()
        .map(|pass| Seat::from_boarding_pass(pass).seat_id())
        .max()
        .unwrap()
}

pub fn day05_2(input: &str) -> u16
{
    let mut seats: Vec<u16> = input
        .lines()
        .map(|pass| Seat::from_boarding_pass(pass).seat_id())
        .collect();
    seats.sort();

    seats.windows(2).fold(0, |acc, el| {
        if el[0] + 2 == el[1] {
            el[0] + 1
        } else {
            acc
        }
    })
}

#[test]
fn test_seat_parsing() {
    vec![("BFFFBBFRRR", 70, 7),
         ("FFFBBBFRRR", 14, 7),
         ("BBFFBBFRLL", 102, 4)]
        .iter()
        .for_each(|x| {
            let seat = Seat::from_boarding_pass(x.0);
            assert_eq!(seat.row, x.1);
            assert_eq!(seat.col, x.2);
        });
}
