pub fn insertion_sort<T: PartialOrd>(list: &mut [T]) {
    let n = list.len();
    
    for i in 1..n {
        let mut j = i;
        while j > 0 && list[j-1] > list[j] {
            list.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_integers() {
        let mut list = [1,4,5,2];
        insertion_sort(&mut list);    
        assert_eq!(list, [1,2,4,5]);
    }

    #[test]
    fn test_sort_vec() {
        let mut list = vec![1,4,5,2];
        insertion_sort(&mut list);    
        assert_eq!(list, [1,2,4,5]);
    }

    #[test]
    fn test_sort_strings() {
        let mut list = vec!["a", "c", "b"];
        insertion_sort(&mut list);    
        assert_eq!(list, ["a", "b", "c"]);
    }

    #[test]
    fn test_already_sorted() {
        let mut list = vec![1,2,3,4];
        insertion_sort(&mut list);    
        assert_eq!(list, [1,2,3,4]);
    }

    #[test]
    fn test_empty_sort() {
        let mut list: Vec<i32> = vec![];
        let expected: [i32; 0] = [];
        insertion_sort(&mut list);    
        assert_eq!(list, expected);
    }

}