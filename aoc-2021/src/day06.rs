use std::collections::VecDeque;

pub fn p1(input: &str, days: usize) -> u64 {
    let mut deque = VecDeque::new();
    for _ in 0..=8 {
        deque.push_back(0);
    }

    let input = input.trim().split(",").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
    input.iter().for_each(|x| {
        deque[*x as usize] += 1;
    });

    for d in 0..days {
        let toadd = deque[0];
        deque.pop_front();
        deque[6] += toadd;
        deque.push_back(toadd);
    }

    deque.iter().sum()
}

#[test]
fn test() {
    assert_eq!(p1("3,4,3,1,2", 80), 5934);
    assert_eq!(p1("3,4,3,1,2", 256), 26984457539);
}
