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
