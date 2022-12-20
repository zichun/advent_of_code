use std::collections::LinkedList;

struct EncryptedFile(Vec<(i64, usize)>);

impl EncryptedFile {
    fn get(&self, ind: usize) -> i64 {
        self.0[ind % self.0.len()].0
    }
    fn mixin(&mut self) {
        for find in 0..self.0.len() {
            let i = self.0.iter().position(|(_, ind)| *ind == find).unwrap();
            let len = self.0.len() as i64;
            let insert_ind = i as i64 + self.0[i].0;
            let insert_ind = insert_ind.rem_euclid(len - 1) as usize;
            let v = self.0[i].0;
            self.0.remove(i);
            self.0.insert(insert_ind, (v, find));
        }
    }
}

fn parse(input: &str) -> EncryptedFile {
    EncryptedFile(
        input.lines().enumerate().map(|(ind, l)| (l.parse().unwrap(), ind)).collect()
    )
}

pub fn part1(input: &str) -> i64 {
    let mut l = parse(input);
    l.mixin();
    let pos = l.0.iter().position(|(x, _)| *x == 0).unwrap();
    l.get(pos + 1000) + l.get(pos + 2000) + l.get(pos + 3000)
}

pub fn part2(input: &str) -> i64 {
    let mut l = parse(input);
    l.0.iter_mut().for_each(|(v, _)| {
        *v = *v * 811589153;
    });
    for _ in 0..10 {
        l.mixin();
    }
    let pos = l.0.iter().position(|(x, _)| *x == 0).unwrap();
    l.get(pos + 1000) + l.get(pos + 2000) + l.get(pos + 3000)
}

#[test]
fn test() {
    let input = "1
2
-3
3
-2
0
4";
    assert_eq!(part1(input), 3);
    assert_eq!(part2(input), 1623178306);
}
