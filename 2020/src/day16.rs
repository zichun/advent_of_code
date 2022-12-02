struct InputParser {
    fields: Vec<(String, Vec<(u32, u32)>)>,
    your_tickets: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>
}
impl InputParser {
    fn parse(input: &str) -> Self {
        let mut fields = Vec::new();
        let mut your_tickets = Vec::new();
        let mut nearby_tickets = Vec::new();

        input.lines()
            .fold(0, |stage, el| {
                if el.trim() == "your ticket:" {
                    return 1;
                } else if el.trim() == "nearby tickets:" {
                    return 2;
                } else if el.trim().is_empty() {
                    return stage;
                } else {
                    if stage == 0 {
                        let mut field_iter = el.split(": ");
                        fields.push(
                            (field_iter.next().unwrap().to_owned(),
                             field_iter.next().unwrap()
                             .split(" or ")
                             .map(|el| {
                                 let mut range = el.split("-").map(|el| el.parse::<u32>().unwrap());
                                 (range.next().unwrap(), range.next().unwrap())
                             }).collect::<Vec<_>>()));
                    } else if stage == 1 {
                        your_tickets = el.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
                    } else if stage == 2 {
                        nearby_tickets.push(
                            el.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>());
                    }
                    stage
                }
            });

        InputParser { fields, your_tickets, nearby_tickets }
    }
}

fn fulfilled(field: &Vec<(u32, u32)>, value: u32) -> bool {
    field.iter().fold(false, |el, (from, to)| {
        el || (value >= *from && value <= *to)
    })
}

pub fn day16_1(input: &str) -> u32 {
    let input = InputParser::parse(input);
    input.nearby_tickets
        .iter()
        .map(|ticket| {
            ticket.iter().filter(|&v| {
                input.fields.iter().fold(true, |el, (_, acc)| {
                    el && !fulfilled(acc, *v)
                })
            }).sum::<u32>()
        })
        .sum()
}

pub fn day16_2(input: &str) -> u64 {
    let input = InputParser::parse(input);
    let valid = input.nearby_tickets
        .iter()
        .filter(|&ticket| {
            ticket.iter().filter(|&v| {
                input.fields.iter().fold(true, |el, (_, acc)| {
                    el && !fulfilled(acc, *v)
                })
            }).count() == 0
        }).collect::<Vec<_>>();

    let n = input.fields.len();

    let mut matching = Vec::new();
    matching.resize(n, Vec::new());
    for i in 0..n {
        for j in 0..n {
            let field = &input.fields[j].1;
            let mut satisfied: bool = true;
            for tickets in &valid {
                satisfied = satisfied && fulfilled(field, tickets[i]);
                if !satisfied {
                    break;
                }
            }
            if satisfied == true {
                // column i is satisfied by field j
                matching[j].push(i);
            }
        }
    }

    let soln = bipartite_bruteforce(&matching);

    input.fields.iter()
        .enumerate()
        .filter(|(_, (s, _))| {
            s.starts_with("departure")
        }).map(|(ind, _)| {
            input.your_tickets[soln[ind]] as u64
        }).product::<u64>()
}

fn bipartite_bruteforce(matching: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut soln = Vec::new();
    let mut used = Vec::new();

    soln.resize(matching.len(), 0);
    used.resize(matching.len(), false);

    let mut map = Vec::new();
    for i in 0..matching.len() {
        map.push((matching[i].len(), i));
    }
    map.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    fn recur(matching: &Vec<Vec<usize>>, depth: usize, used: &mut Vec<bool>, soln: &mut Vec<usize>, map: &Vec<(usize, usize)>) -> bool {
        if depth == matching.len() {
            return true;
        } else {
            let ind = map[depth].1;
            for i in 0..matching[ind].len() {
                if used[matching[ind][i]] {
                    continue;
                }
                used[matching[ind][i]] = true;
                soln[ind] = matching[ind][i];
                if recur(matching, depth + 1, used, soln, map) {
                    return true;
                }
                used[matching[ind][i]] = false;
            }
        }
        false
    }

    if !recur(matching, 0, &mut used, &mut soln, &map) {
        panic!("Can't find a solution");
    }

    soln
}

#[test]
fn test_day16_1() {
    let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    assert_eq!(day16_1(input), 71)
}

#[test]
fn test_day16_2() {
    let input = "departure class: 0-1 or 4-19
departure row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
    assert_eq!(day16_2(input), 132)
}
