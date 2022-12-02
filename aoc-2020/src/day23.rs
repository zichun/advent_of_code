#[derive(Debug)]
struct ListItem {
    left: usize,
    right: usize,
    val: u32
}
struct StaticList {
    data: Vec<ListItem>,
    location: Vec<usize>,
    current: usize
}

impl StaticList {
    fn from(arr: &Vec<u8>, n: usize) -> Self {
        let mut data = Vec::new();
        let mut location = vec![0; n + 1];
        for i in 0..n {
            let val = if i < arr.len() {
                arr[i] as u32
            } else {
                (i + 1) as u32
            };
            location[val as usize] = data.len();
            data.push( ListItem {
                left: if i == 0  { n - 1 } else { i - 1 },
                right: if i == n - 1 { 0 } else { i + 1 },
                val
            });
        }
        StaticList {
            data,
            location,
            current: 0
        }
    }
    fn next(&mut self) {
        let n = self.data.len();
        let mut curr = self.data[self.current].val;

        let left_item = self.data[self.current].right;
        let second_item = self.data[left_item].right;
        let right_item = self.data[second_item].right;

        let right_of = self.data[right_item].right;
        self.data[right_of].left = self.current;
        self.data[self.current].right = right_of;

        let items = (self.data[left_item].val,
                     self.data[second_item].val,
                     self.data[right_item].val);
        loop {
            curr = (curr + n as u32 - 2) % n as u32 + 1;
            if items.0 != curr && items.1 != curr && items.2 != curr {
                break;
            }
        }

        let dest_index = self.location[curr as usize];
        let right_dest = self.data[dest_index].right;
        self.data[left_item].left = dest_index;
        self.data[dest_index].right = left_item;

        self.data[right_dest].left = right_item;
        self.data[right_item].right = right_dest;

        self.current = right_of;
    }
    fn part1(&self) -> String {
        let mut start = self.location[1];
        let mut sb = Vec::new();
        loop {
            start = self.data[start].right;
            if self.data[start].val == 1 {
                break;
            } else {
                sb.push((self.data[start].val as u8 + b'0') as char);
            }
        };
        sb.iter().collect::<String>()
    }
    fn part2(&self) -> u64 {
        let mut start = self.location[1];
        start = self.data[start].right;
        let a = self.data[start].val;
        start = self.data[start].right;
        let b = self.data[start].val;
        a as u64 * b as u64
    }
}

pub fn day23_1(inp: &str) -> String {
    let arr = inp.chars().map(|x| x as u8 - b'0').collect::<Vec<_>>();
    let mut lst = StaticList::from(&arr, 9);
    for _ in 0..100 {
        lst.next();
    }
    lst.part1()
}

pub fn day23_2(inp: &str) -> u64 {
    let arr = inp.chars().map(|x| x as u8 - b'0').collect::<Vec<_>>();
    let mut lst = StaticList::from(&arr, 1000000);
    for _ in 0..10000000 {
        lst.next();
    }
    lst.part2()
}

#[test]
fn test_day23() {
    let inp = "389125467";
    assert_eq!(day23_1(inp), "67384529");
//    assert_eq!(day23_2(inp), 149245887792);
}
