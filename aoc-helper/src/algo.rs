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
