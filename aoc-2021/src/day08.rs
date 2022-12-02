use std::collections::HashMap;

fn sort(chars: &str) -> String {
    let mut v = chars.chars().collect::<Vec<_>>();
    v.sort();
    v.iter().cloned().collect::<String>()
}

fn decode(input: &str) -> HashMap<char, char> {
    let mut char_cnt = HashMap::new();
    let mut char_map = HashMap::new();

    input.chars().for_each(|c| {
        if c.is_ascii_alphabetic() {
            match char_cnt.entry(c) {
                std::collections::hash_map::Entry::Occupied(mut o) => { *o.get_mut() += 1; },
                std::collections::hash_map::Entry::Vacant(v) => { v.insert(1); },
            }
        }
    });
    char_cnt.iter().for_each(|(c, cnt)| {
        char_map.insert(*c, match *cnt {
            4 => 'e',
            6 => 'b',
            9 => 'f',
            _ => '?',
        });
    });
    input.split(" ").for_each(
        |word| {
            if word.len() == 2 {
                let mut iter = char_cnt.iter().filter(|(c, cnt)| **cnt == 8);
                let c = iter.next().unwrap().0;
                if word.contains(*c) {
                    char_map.insert(*c, 'c');
                    char_map.insert(*iter.next().unwrap().0, 'a');
                } else {
                    char_map.insert(*c, 'a');
                    char_map.insert(*iter.next().unwrap().0, 'c');
                }
            } else if word.len() == 4 {
                let mut iter = char_cnt.iter().filter(|(c, cnt)| **cnt == 7);
                let c = iter.next().unwrap().0;
                if word.contains(*c) {
                    char_map.insert(*c, 'd');
                    char_map.insert(*iter.next().unwrap().0, 'g');
                } else {
                    char_map.insert(*c, 'g');
                    char_map.insert(*iter.next().unwrap().0, 'd');
                }
            }
        });

    char_map
}

fn parse(line: &str) -> [u32; 4] {
    let mut inp = line.split(" | ");
    let left = inp.next().unwrap();
    let right = inp.next().unwrap();

    let mut digit_map = HashMap::new();
    digit_map.insert(sort("abcegf"), 0);
    digit_map.insert(sort("cf"), 1);
    digit_map.insert(sort("acdeg"), 2);
    digit_map.insert(sort("acdfg"), 3);
    digit_map.insert(sort("bcdf"), 4);
    digit_map.insert(sort("abdfg"), 5);
    digit_map.insert(sort("abdefg"), 6);
    digit_map.insert(sort("acf"), 7);
    digit_map.insert(sort("abcdefg"), 8);
    digit_map.insert(sort("abcdfg"), 9);

    let char_map = decode(left);
    let mut tr = [0; 4];

    right.split(" ").enumerate().for_each(
        |(ind, word)| {
            let convert = word.chars().map(|c| { char_map.get(&c).unwrap() }).collect::<String>();
            tr[ind] = *digit_map.get(&sort(&convert)).unwrap();
        });

    tr
}

pub fn p1(input: &str) -> u32 {
    input.lines().map(|line| {
        if line.trim().len() > 0 {
            parse(line).iter().filter(|x| {
                **x == 1 || **x == 4 || **x == 7 || **x == 8
            }).count() as u32
        } else {
            0
        }
    }).sum()
}

pub fn p2(input: &str) -> u32 {
    input.lines().map(|line| {
        if line.trim().len() > 0 {
            let p = parse(line);
            p[0] * 1000 + p[1] * 100 + p[2] * 10 + p[3]
        } else {
            0
        }
    }).sum()
}
#[test]
fn test() {
    let input = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;
    assert_eq!(p1(input), 26);
    assert_eq!(p2(input), 61229);

}

