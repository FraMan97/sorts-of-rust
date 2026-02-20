pub fn bubble_sort<T: PartialOrd>(list: &mut [T]) {
    let n = list.len();
    for i in 0..n {
        let mut flag: bool = false;
        for j in 0..n.saturating_sub(1).saturating_sub(i) {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
                flag = true;
            }
        }
        if !flag {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_integers() {
        let mut list = [1,4,5,2];
        bubble_sort(&mut list);    
        assert_eq!(list, [1,2,4,5]);
    }

    #[test]
    fn test_sort_vec() {
        let mut list = vec![1,4,5,2];
        bubble_sort(&mut list);    
        assert_eq!(list, [1,2,4,5]);
    }

    #[test]
    fn test_sort_strings() {
        let mut list = vec!["a", "c", "b"];
        bubble_sort(&mut list);    
        assert_eq!(list, ["a", "b", "c"]);
    }

    #[test]
    fn test_already_sorted() {
        let mut list = vec![1,2,3,4];
        bubble_sort(&mut list);    
        assert_eq!(list, [1,2,3,4]);
    }

    #[test]
    fn test_empty_sort() {
        let mut list: Vec<i32> = vec![];
        let expected: [i32; 0] = [];
        bubble_sort(&mut list);    
        assert_eq!(list, expected);
    }

}