use std::vec;

pub fn merge_sort(x: Vec<i32>) -> Vec<i32> {
    if x.len() <= 1 {
        return x;
    }
    let mid = x.len() / 2;
    let (left, right) = x.split_at(mid);
    let left = merge_sort(left.to_vec());
    let right = merge_sort(right.to_vec());
    let merged = merge(left, right);
    merged
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut merged = vec![0; left.len() + right.len()];
    let mut i_left: usize = 0;
    let mut i_right: usize = 0;
    let mut i_merged: usize = 0;
    while (i_left < left.len()) & (i_right < right.len()) {
        if left[i_left] < right[i_right] {
            merged[i_merged] = left[i_left];
            i_left += 1;
        } else {
            merged[i_merged] = right[i_right];
            i_right += 1;
        }
        i_merged += 1;
    }
    while i_left < left.len() {
        merged[i_merged] = left[i_left];
        i_left += 1;
        i_merged += 1;
    }
    while i_right < right.len() {
        merged[i_merged] = right[i_right];
        i_right += 1;
        i_merged += 1;
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(vec![0; 0], merge_sort(vec![0; 0]));
    }

    #[test]
    fn one() {
        assert_eq!(vec![0], merge_sort(vec![0]));
    }

    #[test]
    fn two() {
        assert_eq!(vec![0, 1], merge_sort(vec![0, 1]));
        assert_eq!(vec![0, 1], merge_sort(vec![1, 0]));
        assert_eq!(vec![1, 1], merge_sort(vec![1, 1]));
    }

    #[test]
    fn many() {
        assert_eq!(vec![0, 1, 2, 3, 4, 5], merge_sort(vec![0, 1, 2, 3, 4, 5]));
        assert_eq!(vec![0, 1, 2, 3, 4, 5], merge_sort(vec![0, 2, 1, 5, 4, 3]));
        assert_eq!(vec![1; 10], merge_sort(vec![1; 10]));
    }
}
