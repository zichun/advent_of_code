fn increase(input: &[u32]) -> u32 {
    input.iter().fold((input[0], 0 as u32), |(prev, acc), el| {
        if prev < *el {
            (*el, acc + 1)
        } else {
            (*el, acc)
        }
    }).1
}

pub fn day01_1(input: &str) -> u32 {
    let input = input.lines().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
    increase(&input)
}

pub fn day01_2(input: &str) -> u32 {
    let input = input.lines().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
    let input = input.windows(3).map(|x| x.iter().sum()).collect::<Vec<_>>();
    increase(&input)
}


#[test]
fn test() {
    assert_eq!(day01_1(r#"199
200
208
210
200
207
240
269
260
263
"#), 7);

    assert_eq!(day01_2
               (r#"199
200
208
210
200
207
240
269
260
263
"#), 5);
}
