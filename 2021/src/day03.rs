use std::collections::HashSet;

pub fn day03_1(input: &str) -> usize {
    let mut n = 0;
    let mut cnt = Vec::new();

    input.lines().for_each(|x| {
        n += 1;
        x.chars().enumerate().for_each(|(ind, c)| {
            if cnt.len() <= ind {
                cnt.push(0);
            }
            cnt[ind] += if c == '1' { 1 } else { 0 };
        });
    });

    let mut this = 0;
    let mut that = 0;
    cnt.iter().enumerate().for_each(|(_ind, cnt)| {
        this <<= 1;
        that <<= 1;
        if *cnt >= (n / 2) {
            this = this + 1;
        } else {
            that = that + 1;
        }
    });

    this * that
}

pub fn day03_2(input: &str) -> u32 {
    let mut n: u32= 0;
    let mut m = 0;
    let set = input.lines().map(|x| {
        n += 1;
        let mut num: u32 = 0;
        m = x.len() as u32;
        x.chars().for_each(|c| {
            num <<= 1;
            if c == '1' {
                num += 1;
            }
        });
        num
    }).collect::<HashSet<_>>();

    fn find(set: &HashSet<u32>, to_find: u32, ind: u32, m: u32) -> u32 {
        let mut one_cnt = 0;
        let n = set.len();
        set.iter().for_each(|x| {
            if *x & (1 << ((m - 1) - ind)) > 0 {
                one_cnt += 1;
            }
        });

        let tokeep = if to_find == 1 {
            if one_cnt * 2 >= n { 1 } else { 0 }
        } else {
            if one_cnt * 2 >= n { 0 } else { 1 }
        };
        let newset = set.iter().filter(|&x| {
            let num = if *x & (1 << ((m - 1) - ind)) > 0 { 1 } else { 0 };
            tokeep == num
        }).map(|x| x.to_owned()).collect::<HashSet<u32>>();

        if newset.len() == 1 {
            return *newset.iter().next().unwrap();
        }

        find(&newset, to_find, ind + 1, m)
    }

    let this = find(&set, 1, 0, m);
    let that = find(&set, 0, 0, m);

    this * that
}

#[test]
fn test() {
    assert_eq!(day03_1(r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#), 198);
    assert_eq!(day03_2(r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#), 230);
}
