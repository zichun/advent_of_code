use linked_list::LinkedList;

#[derive(Clone)]
struct Snail {
    v: LinkedList<(u32, u32)>
}

impl Snail {
    fn from(inp: &str) -> Self {
        let mut v = LinkedList::new();
        let mut d = 0;
        inp.chars().for_each(|c| {
            match c {
                '[' => { d += 1 },
                ']' => { d -= 1 },
                '0'..='9' => {
                    v.push_back(((c as u32) - ('0' as u32), d));
                },
                _ => {}
            }
        });
        Snail { v }
    }
    fn add(&self, s: &Snail) -> Snail {
        let mut v = self.v.clone();

        v.iter_mut().for_each(|ent| {
            ent.1 += 1;
        });
        s.v.iter().for_each(|(num, dep)| {
            v.push_back((*num, *dep + 1));
        });

        Snail { v }
    }

    fn print(&self) {
        let mut cur = 0;
        let len = self.v.len();
        let mut depstack = Vec::new();
        self.v.iter()
            .enumerate()
            .for_each(|(ind, (num, dep))| {
            let mut start = false;
            while cur < *dep {
                cur += 1;
                print!("[");
                start = true;
            }
            if start {
                print!("{},", num);
                depstack.push(cur);
            } else {
                print!("{}", num);
                while depstack.len() > 0 {
                    let precur = depstack.pop().unwrap();
                    if cur == precur {
                        print!("]");
                        cur -= 1;
                    } else {
                        depstack.push(precur);
                        break;
                    }
                }
                if ind + 1 < len {
                    print!(",");
                    depstack.push(cur);
                }
            }
        });
        while cur > 0 {
            cur -= 1;
            print!("]");
        }
        println!("");
    }

    fn reduce(&mut self)  {
        let mut change = true;
        while change {
//            self.print();
            let mut cur = self.v.cursor();
            loop {
                let mut val = 0;
                let mut dep = 0;

                match cur.next() {
                    None => break,
                    Some((v, d)) => {
                        val = *v;
                        dep = *d;
                    }
                }

                if dep >= 5 {
                    if cur.peek_next().unwrap().1 != dep {
                        continue;
                    }
                    let val2 = cur.peek_next().unwrap().0;
                    cur.seek_backward(1);
                    cur.remove();
                    cur.remove();
                    cur.insert((0, dep - 1));
//                    println!("found {} {}", val, val2);
                    // Add left
                    {
                        match cur.prev() {
                            Some((left, dep)) => {
//                                println!("left adding {} to {} at dep {}", val, *left, dep);
                                *left += val;
                            }
                            None => {
                            }
                        }
                    }
                    cur.seek_forward(2);

                    // Add right
                    {
                        match cur.peek_next() {
                            Some((right, dep)) => {
//                                println!("right adding {} to {} at dep {}", val2, *right, dep);
                                *right += val2;
                            }
                            None => {}
                        }
                    }

//                    self.print();
                    cur = self.v.cursor();
                }
            }

            change = false;
            cur = self.v.cursor();
            loop {
                let mut val = 0;
                let mut dep = 0;
                match cur.next() {
                    None => break,
                    Some((v, d)) => {
                        val = *v;
                        dep = *d;
                    }
                }
                if val >= 10 {
//                    println!("splitting {} dep {}", val, dep + 1);
                    cur.seek_backward(1);
                    cur.remove();
                    let toadd = val % 2;
                    cur.insert((val / 2, dep + 1));
                    cur.seek_forward(1);
                    cur.insert((val / 2 + toadd, dep + 1));

                    change = true;
                    break;
                }
            }
        }
    }

    fn magnitude(&self) -> u32 {
        let mut v = self.v.clone();
        let mut cur = v.cursor();

        loop {
            let mut val = 0;
            let mut dep = 0;
            {
                match cur.peek_next() {
                    Some((v, d)) => {
                        val = *v;
                        dep = *d;
                    }
                    None => {
                        panic!("invalid");
                    }
                }
            }

            cur.next();
            if dep == cur.peek_next().unwrap().1 {
                let score = 3 * val + 2 * cur.peek_next().unwrap().0;
                cur.prev();
                cur.remove().unwrap();
                cur.remove().unwrap();

                if dep - 1 == 0 {
                    return score;
                }
                cur.insert((score, dep - 1));
                cur.reset();
            }
        }
    }
}

pub fn p1(inp: &str) -> u32 {
    let snails = inp.lines().map(|l| Snail::from(l)).collect::<Vec<_>>();
    let mut s = snails[0].clone();
    for i in 1..snails.len() {
        s = s.add(&snails[i]);
        s.reduce();
    }
    s.magnitude()
}

pub fn p2(inp: &str) -> u32 {
    let snails = inp.lines().map(|l| Snail::from(l)).collect::<Vec<_>>();
    let mut max = 0;
    for i in 0..snails.len() {
        for j in 0..snails.len() {
            if i == j { continue; }
            let mut s = snails[i].add(&snails[j]);
            s.reduce();
            max = max.max(s.magnitude());
        }
    }
    max
}

#[test]
fn test() {
/*    assert_eq!(Snail::from("[[1,2],[[3,4],5]]").magnitude(), 143);
    assert_eq!(Snail::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]").magnitude(), 3488);
    let mut s = Snail::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    s.reduce();
    assert_eq!(s.v, Snail::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").v);
    println!("{:?}", s.v);*/

    let inp = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;

    assert_eq!(p1(inp), 4140);
    assert_eq!(p2(inp), 3993);
}
