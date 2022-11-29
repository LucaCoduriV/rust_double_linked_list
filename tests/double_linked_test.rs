use double_linked_list::LinkedList;

#[test]
fn create_empty_list() {
    let list:LinkedList<bool> = LinkedList::new();
    assert!(list.is_empty());
}

#[test]
fn create_not_empty_list() {
    let mut list :LinkedList<bool> = LinkedList::new();
    list.push_back(false);
    assert!(!list.is_empty());
}

#[test]
fn list_len_is_1() {
    let mut list :LinkedList<i32> = LinkedList::new();
    list.push_back(1);
    assert!(!list.is_empty());
    assert_eq!(list.len(), 1);
}

#[test]
fn list_len_is_2() {
    let mut list :LinkedList<i32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    assert!(!list.is_empty());
    assert_eq!(list.len(), 2);
}

#[test]
fn list_len_is_3() {
    let mut list :LinkedList<i32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_front(3);
    assert!(!list.is_empty());
    assert_eq!(list.len(), 3);
}

#[test]
fn display_list() {
    let mut list :LinkedList<i32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_front(3);
    assert!(!list.is_empty());
    assert_eq!(list.len(), 3);
    assert_eq!(format!("{}", list), "[3,1,2]");
}

#[test]
fn pop_back() {
    let mut list :LinkedList<i32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_front(3);
    assert_eq!(list.pop_back().unwrap(), 2);
    assert_eq!(list.pop_back().unwrap(), 1);
    assert_eq!(list.pop_back().unwrap(), 3);
}

#[test]
fn pop_back_empty() {
    let mut list :LinkedList<i32> = LinkedList::new();
    assert_eq!(list.pop_back(), None);
}

#[test]
fn pop_front() {
    let mut list :LinkedList<i32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_front(3);
    assert_eq!(list.pop_front().unwrap(), 3);
    assert_eq!(list.pop_front().unwrap(), 1);
    assert_eq!(list.pop_front().unwrap(), 2);
}

#[test]
fn pop_front_empty() {
    let mut list :LinkedList<i32> = LinkedList::new();
    assert_eq!(list.pop_back(), None);
}

#[test]
fn iter_test(){
    let mut list :LinkedList<i32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_front(3);
    let expected = [3,1,2];
    for (i, x) in list.into_iter().enumerate() {
        assert_eq!(x, expected[i]);
    }
}

#[test]
fn ref_fiter_test(){
    let mut list :LinkedList<i32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_front(3);
    let expected = [3,1,2];
    for (i, x) in (&list).into_iter().enumerate() {
        assert_eq!(*x, expected[i]);
    }
}