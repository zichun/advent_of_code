use std::collections::HashMap;

pub fn p1(input: &str, step: usize) -> usize {
    let mut iter = input.lines();

    let input = iter.next().unwrap();

    let mut tmap = HashMap::new();
    while let Some(transform) = iter.next() {
        if transform.trim().len() == 0 {
            continue;
        }
        let mut transform = transform.split(" -> ");
        let from = transform.next().unwrap();
        let to = transform.next().unwrap();
        tmap.insert(from.to_owned(), to.to_owned());
    }


    let mut hm = HashMap::new();
    input.chars().collect::<Vec<_>>().windows(2).map(|x| {
        x.iter().collect::<String>()
    }).for_each(|s| {
        *hm.entry(s).or_insert(0 as usize) += 1;
    });

    for _ in 0..step {
        let mut nhm = HashMap::new();

        hm.iter().for_each(|(k, v)| {
            if tmap.contains_key(k) {
                let new_char = tmap.get(k).unwrap().as_bytes()[0] as char;
                *nhm.entry(format!("{}{}", k.as_bytes()[0] as char, new_char)).or_insert(0) += *v;
                *nhm.entry(format!("{}{}", new_char, k.as_bytes()[1] as char)).or_insert(0) += *v;
            } else {
                nhm.insert(k.clone(), *v);
            }
        });

        hm = nhm;
    }

    let mut char_freq = HashMap::new();
    hm.iter().for_each(|(k, v)| {
        *char_freq.entry(k.as_bytes()[0] as char).or_insert(0) += v;
        *char_freq.entry(k.as_bytes()[1] as char).or_insert(0) += v;
    });

    let mut max = char_freq.iter().max_by(|a, b| a.1.cmp(&b.1))
        .map(|(_k, v)| *v).unwrap();
    let mut min = char_freq.iter().min_by(|a, b| a.1.cmp(&b.1))
        .map(|(_k, v)| *v).unwrap();
    if max % 2 == 1 { max += 1; }
    if min % 2 == 1 { min += 1; }
    max /= 2;
    min /= 2;
    max - min
}


#[test]
fn test() {
    let input = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;
    assert_eq!(p1(input, 10), 1588);
    assert_eq!(p1(input, 40), 2188189693529);
}
