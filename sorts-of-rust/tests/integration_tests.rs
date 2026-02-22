use sorts_of_rust::algorithms::bubble::bubble_sort;
use sorts_of_rust::algorithms::insertion::insertion_sort;
use sorts_of_rust::algorithms::quick::quick_sort;
use sorts_of_rust::algorithms::selection::selection_sort;
use sorts_of_rust::algorithms::merge::merge_sort;

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

#[test]
fn test_quick_sort_integration() {
    let mut list = vec![10, 5, 8, 3, 1];
    quick_sort(&mut list);
    assert_eq!(list, vec![1, 3, 5, 8, 10]);
}

#[test]
fn test_selection_sort_integration() {
    let mut list = vec![10, 5, 8, 3, 1];
    selection_sort(&mut list);
    assert_eq!(list, vec![1, 3, 5, 8, 10]);
}

#[test]
fn test_merge_sort_integration() {
    let mut list = vec![10, 5, 8, 3, 1];
    merge_sort(&mut list);
    assert_eq!(list, vec![1, 3, 5, 8, 10]);
}