pub fn merge_sort<T: PartialOrd + Clone>(list: &mut [T]) {
    let n = list.len();
    if n <= 1 {
        return;
    }

    let mid = n / 2;
    
    merge_sort(&mut list[..mid]);
    merge_sort(&mut list[mid..]);

    merge(list, mid);
}

fn merge<T: PartialOrd + Clone>(list: &mut [T], mid: usize) {
    let left = list[..mid].to_vec(); 
    let right = list[mid..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0; 

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            list[k] = left[i].clone();
            i += 1;
        } else {
            list[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        list[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    while j < right.len() {
        list[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_integers() {
        let mut list = [1,4,5,2];
        merge_sort(&mut list);    
        assert_eq!(list, [1,2,4,5]);
    }

    #[test]
    fn test_sort_vec() {
        let mut list = vec![1,4,5,2];
        merge_sort(&mut list);    
        assert_eq!(list, [1,2,4,5]);
    }

    #[test]
    fn test_sort_strings() {
        let mut list = vec!["a", "c", "b"];
        merge_sort(&mut list);    
        assert_eq!(list, ["a", "b", "c"]);
    }

    #[test]
    fn test_already_sorted() {
        let mut list = vec![1,2,3,4];
        merge_sort(&mut list);    
        assert_eq!(list, [1,2,3,4]);
    }

    #[test]
    fn test_empty_sort() {
        let mut list: Vec<i32> = vec![];
        let expected: [i32; 0] = [];
        merge_sort(&mut list);    
        assert_eq!(list, expected);
    }

}