pub fn median(list: &mut [u8]) -> u8 {
    assert!(!list.is_empty(), "The list cannot be empty");
    quickselect(list, 0, list.len() - 1, list.len() / 2)
    // ALTERNATIVE SOLUTION: just use `select_nth_unstable`
    // *list.select_nth_unstable(list.len() / 2).1
}

/// A selection algorithm that finds the k-th smallest element in an unordered list
/// See https://en.wikipedia.org/wiki/Quickselect
fn quickselect(list: &mut [u8], left: usize, right: usize, k: usize) -> u8 {
    if left == right {
        return list[left];
    }

    let pivot_index = partition(list, left, right);

    if k == pivot_index {
        list[k]
    } else if k < pivot_index {
        quickselect(list, left, pivot_index - 1, k)
    } else {
        quickselect(list, pivot_index + 1, right, k)
    }
}

fn partition(list: &mut [u8], left: usize, right: usize) -> usize {
    let pivot = list[right];
    let mut store_index = left;
    for i in left..right {
        if list[i] < pivot {
            list.swap(store_index, i);
            store_index += 1;
        }
    }
    list.swap(right, store_index);
    store_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn empty_list_can_have_no_median() {
        median(&mut []);
    }

    #[test]
    fn single_element_is_the_median() {
        assert_eq!(1, median(&mut [1]));
    }

    #[test]
    fn two_elements_the_second_is_median() {
        assert_eq!(2, median(&mut [1, 2]));
    }

    #[test]
    fn three_elements() {
        assert_eq!(2, median(&mut [1, 2, 3]));
    }

    #[test]
    fn four_elements() {
        assert_eq!(3, median(&mut [4, 1, 2, 3]));
    }

    #[test]
    fn long_list() {
        assert_eq!(5, median(&mut [1, 6, 9, 10, 0, 3, 4, 5, 8, 7, 2]));
    }
}
