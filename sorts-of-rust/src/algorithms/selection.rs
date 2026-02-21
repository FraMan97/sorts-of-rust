pub fn selection_sort<T: PartialOrd>(list: &mut [T]) {
    let n = list.len();
    
    for i in 0..n {
        let remaining_part = &list[i..];
        
        let min_index_in_remaining = remaining_part
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(idx, _)| idx);

        if let Some(min_idx) = min_index_in_remaining {
            list.swap(i, i + min_idx);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_integers() {
        let mut list = [1,4,5,2];
        selection_sort(&mut list);    
        assert_eq!(list, [1,2,4,5]);
    }

    #[test]
    fn test_sort_vec() {
        let mut list = vec![1,4,5,2];
        selection_sort(&mut list);    
        assert_eq!(list, [1,2,4,5]);
    }

    #[test]
    fn test_sort_strings() {
        let mut list = vec!["a", "c", "b"];
        selection_sort(&mut list);    
        assert_eq!(list, ["a", "b", "c"]);
    }

    #[test]
    fn test_already_sorted() {
        let mut list = vec![1,2,3,4];
        selection_sort(&mut list);    
        assert_eq!(list, [1,2,3,4]);
    }

    #[test]
    fn test_empty_sort() {
        let mut list: Vec<i32> = vec![];
        let expected: [i32; 0] = [];
        selection_sort(&mut list);    
        assert_eq!(list, expected);
    }

}