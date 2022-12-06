use std::collections::VecDeque;

#[derive(Default)]
struct SlidingWindow {
    map: [usize; 32],
    queue: VecDeque<char>,
    pub unique_cnt: usize,
}

impl SlidingWindow {
    fn add(&mut self, lower_alpha: char) {
        self.queue.push_back(lower_alpha);
        self.map[lower_alpha as usize - 'a' as usize] += 1;
        if self.map[lower_alpha as usize - 'a' as usize] == 1 {
            self.unique_cnt += 1;
        }
    }
    fn pop(&mut self) {
        let lower_alpha = self.queue.pop_front().unwrap();
        self.map[lower_alpha as usize - 'a' as usize] -= 1;
        if self.map[lower_alpha as usize - 'a' as usize] == 0 {
            self.unique_cnt -= 1;
        }
    }
}

fn process(input: &str, window_size: usize) -> usize{
    let mut sliding_window = SlidingWindow::default();

    input.chars()
        .enumerate()
        .fold(0, |acc, (ind, c)| {
            if acc > 0 {
                acc
            } else if ind < window_size {
                sliding_window.add(c);
                acc
            } else {
                if sliding_window.unique_cnt == window_size {
                    ind
                } else {
                    sliding_window.add(c);
                    sliding_window.pop();
                    acc
                }
            }
        })
}

pub fn part1(input: &str) -> usize {
    process(input, 4)
}

pub fn part2(input: &str) -> usize {
    process(input, 14)
}

#[test]
fn test() {
    let tests = vec![
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26)
    ];
    tests.iter().for_each(|(inp, ans, ans2)| {
        assert_eq!(part1(inp), *ans);
        assert_eq!(part2(inp), *ans2);
    });
}
