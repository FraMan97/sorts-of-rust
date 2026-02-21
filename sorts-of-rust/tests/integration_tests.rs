use sorts_of_rust::algorithms::bubble::bubble_sort;
use sorts_of_rust::algorithms::insertion::insertion_sort;

#[test]
fn test_bubble_sort_integration() {
    let mut list = vec![10, 5, 8, 3, 1];
    bubble_sort(&mut list);
    assert_eq!(list, vec![1, 3, 5, 8, 10]);
}

#[test]
fn test_insertion_sort_integration() {
    let mut list = vec![10, 5, 8, 3, 1];
    insertion_sort(&mut list);
    assert_eq!(list, vec![1, 3, 5, 8, 10]);
}