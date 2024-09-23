pub fn mode(list: &mut [u8]) -> u8 {
    assert!(!list.is_empty(), "The list cannot be empty");

    let mut mode_value = list[0];

    let mut map = std::collections::HashMap::new();
    map.insert(mode_value, 1);

    for &elem in list[1..].iter() {
        let count = map.entry(elem).or_default();
        *count += 1;
        if *count > map.get(&mode_value).copied().unwrap() {
            mode_value = elem;
        }
    }

    mode_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn empty_list_can_have_no_mode() {
        mode(&mut []);
    }

    #[test]
    fn single_element_is_the_mode() {
        assert_eq!(1, mode(&mut [1]));
    }

    #[test]
    fn two_diff_elements_the_first_is_mode() {
        assert_eq!(1, mode(&mut [1, 2]));
    }

    #[test]
    fn two_same_elements_the_first_is_mode() {
        assert_eq!(2, mode(&mut [2, 2]));
    }

    #[test]
    fn three_elements() {
        assert_eq!(2, mode(&mut [1, 2, 2]));
    }

    #[test]
    fn long_list() {
        assert_eq!(3, mode(&mut [1, 6, 3, 10, 0, 3, 4, 5, 8, 7, 2]));
    }
}
