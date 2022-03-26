use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Less;

pub fn find<T>(array: &[T], key: T) -> Option<usize>
where
    T: Ord,
{
    // array.binary_search(&key).ok()
    let mut size = array.len();
    let mut left = 0;
    let mut right = size;

    while left < right {
        let mid_idx = left + size / 2;

        let mid = &array[mid_idx];
        let cmp = mid.cmp(&key);

        if cmp == Less {
            left = mid_idx + 1;
        } else if cmp == Greater {
            right = mid_idx;
        } else {
            return Some(mid_idx);
        }
        size = right - left;
    }
    None
}
