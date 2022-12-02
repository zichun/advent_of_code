use std::collections::HashMap;

pub fn p1(one: usize, two: usize) -> usize {
    let mut diecnt = 0;
    let mut die = 100;

    fn next(die: &mut usize) -> usize {
        *die += 1;
        *die = (*die - 1) % 100 + 1;
        *die
    }

    let mut pos = [one, two];
    let mut turn = 0;
    let mut score = [0, 0];

    loop {
        diecnt += 3;
        let add = next(&mut die) + next(&mut die) + next(&mut die);

        pos[turn] += add;
        pos[turn] = (pos[turn] - 1) % 10 + 1;
        score[turn] += pos[turn];
        turn = 1 - turn;

        if score[1 - turn] >= 1000 {
            return score[turn] * diecnt;
        }
    }
}


fn hash(one: u64, two: u64, one_score: u64, two_score: u64) -> u64 {
    one_score * 22 * 10 * 10 + two_score * 10 * 10 + (one - 1) * 10 + (two - 1)
}
fn unhash(mut key: u64) -> (u64, u64, u64, u64) {
    let two = key % 10 + 1;
    key /= 10;
    let one = key % 10 + 1;
    key /= 10;
    let two_score = key % 22;
    key /= 22;
    let one_score = key;
    (one, two, one_score, two_score)
}

pub fn p2(one: u64, two: u64) -> u64 {
    let mut landing = HashMap::new();

    landing.insert(hash(one, two, 0, 0), 1);

    let mut freq = [0; 10];
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                freq[i + j + k] += 1;
            }
        }
    }
    let mut turn = 0;
    let mut win = [0, 0];

    while !landing.is_empty() {
        let mut nlanding = HashMap::new();

        for (key, cnt) in landing.iter() {
            let (one, two, one_score, two_score) = unhash(*key);

            for i in 3..=9 {
                let mut p = [one, two];
                let mut s = [one_score, two_score];

                p[turn] += i;
                p[turn] = (p[turn] - 1) % 10 + 1;
                s[turn] += p[turn];
                let ucnt = cnt * freq[i as usize];

                if s[turn] >= 21 {
                    win[turn] += ucnt;
                } else {
                    *nlanding.entry(hash(p[0], p[1], s[0], s[1])).or_insert(0) += ucnt;
                }
            }
        }
        landing = nlanding;
        turn = 1 - turn;
    }

    win[0].max(win[1])
}

#[test]
fn test() {
    assert_eq!(p1(4, 8), 739785);
    assert_eq!(p2(4, 8), 444356092776315);
}
