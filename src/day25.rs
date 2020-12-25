struct TransformIter {
    subject: u64,
    value: u64
}

impl Iterator for TransformIter {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.value = (self.value * self.subject) % 20201227;
        Some(self.value)
    }
}

fn get_loop(public: u64) -> u64 {
    let c = TransformIter { subject: 7, value: 1 };
    1 + c.enumerate().skip_while(|(_, val)| *val != public).next().unwrap().0 as u64
}

pub fn day25_1(card_public: u64, door_public: u64) -> u64 {
    let door_loop = get_loop(door_public);

    (TransformIter { subject: card_public, value: 1 }).skip((door_loop - 1) as usize).next().unwrap()
}

#[test]
fn day25() {
    assert_eq!(get_loop(5764801), 8);
    assert_eq!(get_loop(14897079), 11);
    assert_eq!(day25_1(5764801, 17807724), 14897079);
}
