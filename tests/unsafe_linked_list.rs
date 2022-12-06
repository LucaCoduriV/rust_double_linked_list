use double_linked_list::unsafe_linked_list::LinkedList;

#[test]
fn push_back_element() {
    let mut list: LinkedList<i32> = LinkedList::new();

    list.push_back(1);
    assert_eq!(list.len(), 1);
    list.push_back(2);
    assert_eq!(list.len(), 2);
}

#[test]
fn push_front_element() {
    let mut list: LinkedList<i32> = LinkedList::new();

    list.push_front(1);
    assert_eq!(list.len(), 1);
    list.push_front(2);
    assert_eq!(list.len(), 2);
}

#[test]
fn pop_back_element() {
    let mut list: LinkedList<i32> = LinkedList::new();

    list.push_back(1);
    list.push_back(2);

    let el1 = list.pop_back().unwrap();
    assert_eq!(el1, 2);
    assert_eq!(list.len(), 1);
    let el2 = list.pop_back().unwrap();
    assert_eq!(el2, 1);
    assert_eq!(list.len(), 0);
}

#[test]
fn pop_front_element() {
    let mut list: LinkedList<i32> = LinkedList::new();

    list.push_back(1);
    list.push_back(2);

    let el1 = list.pop_front().unwrap();
    assert_eq!(el1, 1);
    assert_eq!(list.len(), 1);
    let el2 = list.pop_front().unwrap();
    assert_eq!(el2, 2);
    assert_eq!(list.len(), 0);
}

#[test]
fn iter(){
    let mut list:LinkedList<i32> = LinkedList::new();
    list.push_back(0);
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    for (i, el) in list.iter().enumerate(){
        assert_eq!(i as i32, *el);
    }

}

#[test]
fn into_iter(){
    let mut list:LinkedList<i32> = LinkedList::new();
    list.push_back(0);
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    for (i, el) in list.into_iter().enumerate(){
        assert_eq!(i as i32, el);
    }

}

#[test]
fn iter_mut(){
    let mut list:LinkedList<i32> = LinkedList::new();
    list.push_back(0);
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    for (_, el) in list.iter_mut().enumerate(){
        *el = 10;
    }

    for el in list.into_iter(){
        assert_eq!(10, el);
    }

}

