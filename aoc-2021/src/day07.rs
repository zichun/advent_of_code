pub fn p1(input: &str) -> i64 {
    let mut input = input.trim().split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    input.sort();

    let mut min = input.last().unwrap() * input.last().unwrap();
    for i in input.iter() {
        min = min.min(input.iter().map(|x| (x - *i).abs()).sum())
    }

    min
}

pub fn p2(input: &str) -> i64 {
    let mut input = input.trim().split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    input.sort();

    let first = input.first().unwrap();
    let last = input.last().unwrap();
    let mut min = last * last * last;
    for i in *first..=*last
    {
        min = min.min(input.iter().map(|x| {
            let diff = (x - i).abs();
            (diff * (diff + 1)) / 2
        }).sum());
    }


    min
}

#[test]
fn test() {
    assert_eq!(p1("16,1,2,0,4,2,7,1,2,14"), 37);
    assert_eq!(p2("16,1,2,0,4,2,7,1,2,14"), 168);
}
