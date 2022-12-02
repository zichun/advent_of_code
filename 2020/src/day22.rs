use std::collections::VecDeque;
use std::collections::HashSet;

fn score(deck: &VecDeque<u32>) -> u32 {
    deck.iter().rev().enumerate().fold(0, |acc, (index, val) | {
        acc + *val * (index + 1) as u32
    })
}
pub fn day22_1(input: &str) -> u32 {
    let mut decks = input.split("\n\n")
        .flat_map(|x| x.split("\r\n\r\n"))
        .map(|deck| {
            deck.lines().skip(1).map(|card| card.parse::<u32>().unwrap()).collect::<VecDeque<_>>()
        }).collect::<Vec<_>>();

    while !decks[0].is_empty() && !decks[1].is_empty() {
        let (t0, t1) = (decks[0].pop_front().unwrap(), decks[1].pop_front().unwrap());
        if t0 > t1 {
            decks[0].push_back(t0);
            decks[0].push_back(t1);
        } else {
            decks[1].push_back(t1);
            decks[1].push_back(t0);
        }
    }
    let winner: &VecDeque<u32> = if decks[0].is_empty() { &decks[1] } else { &decks[0] };
    score(winner)
}

fn recursive_combat(deck0: &mut VecDeque<u32>, deck1: &mut VecDeque<u32>) -> (usize, u32) {
    let mut history = HashSet::new();
    while !deck0.is_empty() && !deck1.is_empty() {
        let mut d0 = deck0.iter().map(|x| x.to_owned()).collect::<Vec<_>>();
        d0.push(0);
        d0.extend(deck1.iter().map(|x| x.to_owned()));

        if history.contains(&d0) {
            return (0, 0);
        } else {
            history.insert(d0);
        }

        let (t0, t1) = (deck0.pop_front().unwrap(), deck1.pop_front().unwrap());
        let winner = if deck0.len() >= t0 as usize && deck1.len() >= t1 as usize{
            recursive_combat(&mut deck0.iter().take(t0 as usize).map(|x| x.to_owned()).collect::<VecDeque<_>>(),
                             &mut deck1.iter().take(t1 as usize).map(|x| x.to_owned()).collect::<VecDeque<_>>()).0
        } else if t0 > t1 {
            0
        } else {
            1
        };

        if winner == 0 {
            deck0.push_back(t0);
            deck0.push_back(t1);
        } else {
            deck1.push_back(t1);
            deck1.push_back(t0);
        }
    }
    if !deck0.is_empty() {
        (0, score(&deck0))
    } else {
        (1, score(&deck1))
    }
}

pub fn day22_2(input: &str) -> u32 {
    let decks = input.split("\n\n")
        .flat_map(|x| x.split("\r\n\r\n"))
        .map(|deck| {
            deck.lines().skip(1).map(|card| card.parse::<u32>().unwrap()).collect::<VecDeque<_>>()
        }).collect::<Vec<_>>();
    let mut d0 = decks[0].clone();
    let mut d1 = decks[1].clone();
    recursive_combat(&mut d0, &mut d1).1
}

#[test]
fn test_day22_1() {
    let inp = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
    assert_eq!(day22_1(inp), 306);
    assert_eq!(day22_2(inp), 291);
}
