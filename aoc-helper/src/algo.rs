use std::{cmp::Ordering, collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet, VecDeque}};
use std::hash::Hash;

pub fn bsearch<I, F>(mut left: I, mut right: I, mut test: F) -> I
where
    I: num::Integer + Copy + From<u8>,
    F: FnMut(I) -> bool,
{
    while left < right {
        let mid = (left + right) / I::from(2);
        if test(mid) {
            left = mid + I::one();
        } else {
            right = mid - I::one();
        }
    }
    left
}

pub fn tsearch<I, F>(mut left: I, mut right: I, mut cmp: F) -> I
where
    I: num::Integer + Copy + From<u8>,
    F: FnMut(I, I) -> bool,
{
    while right > left {
        let left_split = left + (right - left) / I::from(3);
        let right_split = right - (right - left) / I::from(3);
        if cmp(left_split, right_split) {
            left = left_split + I::one();
        } else {
            right = right_split - I::one();
        }
    }
    left
}

pub fn overlap<T>(from0: T, to0: T, from1: T, to1: T) -> bool
where T: Ord {
    (from0 <= from1 && from1 <= to0) ||
        (from0 <= to1 && to1 <= to0) ||
        (from1 <= from0 && to1 >= to0)
}

pub fn extended_euclidean<T>(a: T, b: T) -> (T, T, T)
where T: std::cmp::PartialEq<isize> + std::ops::Rem<Output = T> + std::ops::Div
    + From<isize>
    + std::ops::Sub<<<T as std::ops::Div>::Output as std::ops::Mul<T>>::Output, Output = T> + Copy, <T as std::ops::Div>::Output: std::ops::Mul<T>
{
    if b == 0 {
        return (a, T::from(1), T::from(0));  // Base case: gcd(a, 0) = a, and x = 1, y = 0
    }
    let (g, x1, y1) = extended_euclidean(b, a % b);
    let x = y1;
    let y = x1 - (a / b) * y1;
    (g, x, y)
}

#[derive(PartialEq, Eq)]
pub struct InnerDijk<T> {
    t: T,
    dist: usize,
}
impl<T: Ord> Ord for InnerDijk<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
            .then_with(|| other.t.cmp(&self.t))
    }
}
impl<T: Ord> PartialOrd for InnerDijk<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
pub struct Dijkstra<T> {
    heap: BinaryHeap<InnerDijk<T>>,
    dists: HashMap<T, usize>,
}
impl <T> Dijkstra<T>
where T: std::hash::Hash + Clone + Copy + Eq + Ord
{
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            dists: HashMap::new()
        }
    }
    pub fn visit(&mut self, t: T, dist: usize) -> bool {
        if dist < *self.dists.entry(t).or_insert(usize::MAX) {
            self.dists.insert(t, dist);
            self.heap.push(InnerDijk {
                t,
                dist
            });
            true
        } else {
            false
        }
    }
    pub fn pop(&mut self) -> Option<(T, usize)> {
        self.heap.pop().map(|InnerDijk { t, dist }| (t, dist))
    }
}

impl<T> Default for Dijkstra<T>
where T: std::hash::Hash + Clone + Copy + Eq + Ord
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct Bfs<T> {
    q: VecDeque<(T, usize)>,
    dist: HashMap<T, usize>,
}
impl<T> Bfs<T>
where T: std::hash::Hash + Clone + Copy + Eq
{
    pub fn new() -> Self {
        Self {
            q: VecDeque::new(),
            dist: HashMap::new()
        }
    }
    pub fn visit(&mut self, t: T, dist: usize) -> bool {
        if let Entry::Vacant(e) = self.dist.entry(t) {
            e.insert(dist);
            self.q.push_back((t, dist));
            true
        } else {
            false
        }
    }
    pub fn pop(&mut self) -> Option<(T, usize)> {
        self.q.pop_front()
    }
    pub fn get_distance(&self) -> HashMap<T, usize> {
        self.dist.clone()
    }
}

impl<T> Default for Bfs<T>
where T: std::hash::Hash + Clone + Copy + Eq
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(PartialEq, Eq)]
pub struct InnerNode<T> {
    t: T,
    pub g_score: usize,
    pub f_score: usize,
}

impl<T: Ord> Ord for InnerNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
            .then_with(|| other.t.cmp(&self.t))
    }
}

impl<T: Ord> PartialOrd for InnerNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct AStar<T> {
    heap: BinaryHeap<InnerNode<T>>,
    g_scores: HashMap<T, usize>,
}

impl<T> AStar<T>
where
    T: Hash + Clone + Eq + Ord,
{
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            g_scores: HashMap::new(),
        }
    }

    /// t: The node to visit
    /// g: The actual cost to reach this node from the start
    /// h: The heuristic (estimated cost from this node to the goal)
    pub fn visit(&mut self, t: T, g: usize, h: usize) -> bool {
        if g < *self.g_scores.entry(t.clone()).or_insert(usize::MAX) {
            self.g_scores.insert(t.clone(), g);

            self.heap.push(InnerNode {
                t,
                g_score: g,
                f_score: g + h,
            });
            true
        } else {
            false
        }
    }

    pub fn pop(&mut self) -> Option<(T, usize)> {
        self.heap.pop().map(|InnerNode { t, g_score, .. }| (t, g_score))
    }
}

impl<T> Default for AStar<T>
where
    T: Hash + Clone + Copy + Eq + Ord,
{
    fn default() -> Self {
        Self::new()
    }
}
