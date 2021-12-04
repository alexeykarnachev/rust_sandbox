fn heapify(x: &mut [i32]) {
    for parent_ind in (0..x.len() / 2).rev() {
        heapify_from_parent(x, parent_ind);
    }
}

fn heapify_from_parent(x: &mut [i32], parent_ind: usize) {
    let left_ind = parent_ind * 2 + 1;
    let right_ind = parent_ind * 2 + 2;

    let greatest_ind = if left_ind < x.len() && x[left_ind] > x[parent_ind] {
        left_ind
    } else {
        parent_ind
    };

    let greatest_ind = if right_ind < x.len() && x[right_ind] > x[greatest_ind] {
        right_ind
    } else {
        greatest_ind
    };

    if greatest_ind != parent_ind {
        x.swap(greatest_ind, parent_ind);
        heapify_from_parent(x, greatest_ind);
    }
}

pub fn heap_sort(x: &mut [i32]) {
    heapify(x);
    for i in (0..x.len()).rev() {
        x.swap(0, i);
        let (x, _) = x.split_at_mut(i);
        heapify_from_parent(x, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_heapify() {
        let mut x = [1, 2, 5, 0, 2, 1, 5, 3, 4];
        heapify(&mut x);
        for parent_ind in 0..x.len() / 2 {
            let left_ind = parent_ind * 2 + 1;
            let right_ind = parent_ind * 2 + 2;
            assert!(x[parent_ind] >= x[left_ind]);
            assert!(x[parent_ind] >= x[right_ind]);
        }
    }

    #[test]
    fn empty() {
        let mut x = vec![0; 0];
        heap_sort(x.as_mut_slice());
        assert_eq!(vec![0; 0], x);
    }

    #[test]
    fn one() {
        let mut x = vec![0];
        heap_sort(x.as_mut_slice());
        assert_eq!(vec![0], x);
    }

    #[test]
    fn two() {
        let mut x = vec![0, 1];
        heap_sort(x.as_mut_slice());
        assert_eq!(vec![0, 1], x);

        let mut x = vec![1, 0];
        heap_sort(x.as_mut_slice());
        assert_eq!(vec![0, 1], x);

        let mut x = vec![1, 1];
        heap_sort(x.as_mut_slice());
        assert_eq!(vec![1, 1], x);
    }

    #[test]
    fn many() {
        let mut x = vec![0, 1, 2, 3, 4, 5];
        heap_sort(x.as_mut_slice());
        assert_eq!(vec![0, 1, 2, 3, 4, 5], x);

        let mut x = vec![0, 2, 1, 5, 4, 3];
        heap_sort(x.as_mut_slice());
        assert_eq!(vec![0, 1, 2, 3, 4, 5], x);

        let mut x = vec![1; 10];
        heap_sort(x.as_mut_slice());
        assert_eq!(vec![1; 10], x);
    }
}
