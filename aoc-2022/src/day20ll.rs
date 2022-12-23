struct CyclicLinkedList<T> {
    list: Vec<(T, usize, usize)>,
}

impl<T> CyclicLinkedList<T> {
    fn from(inp: &[T]) -> CyclicLinkedList<T>
    where T: Copy {
        let mut list = Vec::new();
        for i in 0..inp.len() {
            list.push((inp[i],
                       if i == 0 { inp.len() - 1 } else { i - 1 },
                       if i == inp.len() - 1 { 0 } else { i + 1 }));
        }
        Self { list }
    }
    fn move_index(&mut self, ind: usize, times: isize) {
        let mut new_ind = ind;
        if times == 0 {
            return;
        }
        for _ in 0..times.abs() {
            new_ind = if times < 0 {
                self.list[new_ind].1
            } else {
                self.list[new_ind].2
            };
        }
        if times < 0 {
            new_ind = self.list[new_ind].1;
        }

        // remove ind
        let (left, right) = (self.list[ind].1, self.list[ind].2);
        self.list[left].2 = right;
        self.list[right].1 = left;

        // insert ind into new_ind
        let new_right_ind = self.list[new_ind].2;
        self.list[ind].1 = new_ind;
        self.list[ind].2 = new_right_ind;
        self.list[new_right_ind].1 = ind;
        self.list[new_ind].2 = ind;
    }
    fn as_list(&self) -> Vec<T> where
    T: Copy {
        let mut ind = 0;
        let mut tr = Vec::new();
        loop {
            tr.push(self.list[ind].0);
            ind = self.list[ind].2;
            if ind == 0 {
                break;
            }
        }
        tr
    }
}

fn part1(input: &str) -> isize {
    let inp = input.lines().enumerate().map(|(ind, l)| (l.parse::<isize>().unwrap(), ind)).collect::<Vec<_>>();
    let mut list = CyclicLinkedList::from(&inp);

    for i in 0..inp.len() {
        list.move_index(i, inp[i].0);
    }
    let l = list.as_list();
    println!("{:?}", l);
    let pos = l.iter().position(|(val, _)| *val == 0).unwrap();
    l[(pos + 1000) % l.len()].0 + l[(pos + 2000) % l.len()].0 + l[(pos + 3000) % l.len()].0
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
}
