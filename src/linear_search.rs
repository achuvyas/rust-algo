pub fn linear_search<T: PartialEq>(haystack: &[T], needle: &T) -> Option<usize> {
    for (index, item) in haystack.iter().enumerate() {
        if item == needle {
            return Some(index);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search_found() {
        let array = vec![1, 2, 3, 4, 5];
        let target = 3;
        let result = linear_search(&array, &target);
        assert_eq!(result, Some(2));
    }
    #[test]
    fn test_linear_search_not_found() {
        let array = vec![1, 2, 3, 4, 5];
        let target = 6;
        let result = linear_search(&array, &target);
        assert_eq!(result, None);
    }

}
