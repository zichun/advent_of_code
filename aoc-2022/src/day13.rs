#[derive(Clone, PartialEq, Debug)]
enum Value {
    List(String, Vec<Value>),
    Number(String, u32),
}

impl Value {
    fn to_string(&self) -> String {
        match self {
            Value::List(s, _) => s.to_owned(),
            Value::Number(s, _) => s.to_owned(),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        fn cmp(left: &[Value], right: &[Value]) -> std::cmp::Ordering {
            for i in 0..left.len() {
                if i >= right.len() || left[i] > right[i] {
                    return std::cmp::Ordering::Greater;
                }
                if left[i] < right[i] {
                    return std::cmp::Ordering::Less;
                }
            }
            if left.len() == right.len() {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Less
            }
        }


        match self {
            Value::List(_, left_collect) => {
                match other {
                    Value::List(_, right_collect) => Some(cmp(left_collect, right_collect)),
                    Value::Number(_, _) => self.partial_cmp(&Value::List(String::new(), vec![other.clone()])),
                }
            }
            Value::Number(_, left) => {
                match other {
                    Value::Number(_, right) => left.partial_cmp(right),
                    Value::List(_, _) => Value::List(String::new(), vec![self.clone()]).partial_cmp(other),
                }
            }
        }
    }
}

impl From<&str> for Value {
    fn from(line: &str) -> Self {
        if line.as_bytes()[0] == '[' as u8 {
            let len = line.len();
            let inner = line.chars().skip(1).take(len - 2).collect::<String>();

            let mut collection = Vec::new();
            let mut opening = 0;
            let mut buf = String::new();
            for i in 0..inner.len() {
                if opening == 0 && inner.as_bytes()[i] == ',' as u8 {
                    collection.push(Value::from(buf.as_str()));
                    buf.clear();
                    continue;
                }
                buf.push(inner.as_bytes()[i] as char);
                if inner.as_bytes()[i] == '[' as u8 {
                    opening += 1;
                } else if inner.as_bytes()[i] == ']' as u8 {
                    opening -= 1;
                }
            }
            if buf.len() > 0 {
                collection.push(Value::from(buf.as_str()));
            }

            Value::List(line.to_owned(), collection)
        } else {
            Value::Number(line.to_owned(), line.parse::<u32>().unwrap())
        }
    }
}

fn parse(input: &str) -> Vec<(Value, Value)> {
    input.split("\n\n").map(|pairs| {
        let mut lines = pairs.lines();
        (Value::from(lines.next().unwrap()), Value::from(lines.next().unwrap()))
    }).collect()
}

pub fn part1(input: &str) -> usize {
    let input = parse(input);
    input.iter().enumerate().filter_map(|(ind, pair)| {
        if pair.0 <= pair.1 {
            Some(ind + 1)
        } else {
            None
        }
    }).sum()
}

pub fn part2(input: &str) -> usize {
    let dividers = vec!["[[2]]".to_owned(), "[[6]]".to_owned()];
    let mut input = parse(input).iter().flat_map(|x| vec![x.0.clone(), x.1.clone()]).collect::<Vec<_>>();
    dividers.iter().for_each(|i| input.push(Value::from(i.as_str())));

    input.sort_by(|a, b| a.partial_cmp(b).unwrap());

    input.iter().enumerate()
        .filter(|(_, v)| dividers.contains(&v.to_string()))
        .map(|(ind, _)| ind + 1)
        .product()
}

#[test]
fn test() {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[[7],[7],[7],[7]]
[[7],[7],[7]]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    assert_eq!(part1(input), 13);
    assert_eq!(part2(input), 140);
}
