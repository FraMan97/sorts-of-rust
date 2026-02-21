pub fn quick_sort<T: PartialOrd>(list: &mut [T]) {
    let n = list.len();
    if n <= 1 { return; }

    let pivot_idx = partition(list);

    quick_sort(&mut list[0..pivot_idx]);
    quick_sort(&mut list[pivot_idx + 1..n]);
}

fn partition<T: PartialOrd>(list: &mut [T]) -> usize {
    let n = list.len();
    let pivot_idx = n / 2;
    
    list.swap(pivot_idx, n - 1);
    
    let mut i = 0;
    for j in 0..n - 1 {
        if list[j] <= list[n - 1] {
            list.swap(i, j);
            i += 1;
        }
    }
    
    list.swap(i, n - 1);
    i
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_integers() {
        let mut list = [1,4,5,2];
        quick_sort(&mut list);    
        assert_eq!(list, [1,2,4,5]);
    }

    #[test]
    fn test_sort_vec() {
        let mut list = vec![1,4,5,2];
        quick_sort(&mut list);    
        assert_eq!(list, [1,2,4,5]);
    }

    #[test]
    fn test_sort_strings() {
        let mut list = vec!["a", "c", "b"];
        quick_sort(&mut list);    
        assert_eq!(list, ["a", "b", "c"]);
    }

    #[test]
    fn test_already_sorted() {
        let mut list = vec![1,2,3,4];
        quick_sort(&mut list);    
        assert_eq!(list, [1,2,3,4]);
    }

    #[test]
    fn test_empty_sort() {
        let mut list: Vec<i32> = vec![];
        let expected: [i32; 0] = [];
        quick_sort(&mut list);    
        assert_eq!(list, expected);
    }

}