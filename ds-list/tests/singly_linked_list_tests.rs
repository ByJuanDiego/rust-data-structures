use ds_list::{List, SinglyLinkedList};

#[cfg(test)]
mod tests_singly_linked_list {
    use super::*;

    #[test]
    fn empty_list_test() {
        let fl: SinglyLinkedList<i32> = SinglyLinkedList::new();
        assert_eq!(fl.len(), 0);
    }

    #[test]
    fn pop_front_in_empty_list_test() {
        let mut fl: SinglyLinkedList<i32> = SinglyLinkedList::new();
        assert!(fl.pop_front().is_err());
    }

    #[test]
    fn pop_front_in_non_empty_list_test() {
        let mut fl: SinglyLinkedList<i32> = SinglyLinkedList::new();
        fl.push_front(10);
        fl.push_front(20);
        fl.push_front(30);

        assert_eq!(fl.pop_front().unwrap(), 30);
        assert_eq!(fl.pop_front().unwrap(), 20);
        assert_eq!(fl.pop_front().unwrap(), 10);
    }


    #[test]
    fn push_back_test() {
        let mut fl: SinglyLinkedList<i32> = SinglyLinkedList::new();
        fl.push_back(10);
        fl.push_back(20);
        fl.push_back(30);

        assert_eq!(fl.len(), 3);
    }

    #[test]
    fn get_empty_list_test() {
        let mut dl: SinglyLinkedList<i32> = SinglyLinkedList::new();

        assert_eq!(dl.get(0), None);
    }

    #[test]
    fn get_unwrap_test() {
        let mut fl: SinglyLinkedList<i32> = SinglyLinkedList::new();
        fl.push_back(10);
        fl.push_back(20);
        fl.push_back(30);

        assert_eq!(fl.get(0), Some(10));
        assert_eq!(fl.get(1), Some(20));
        assert_eq!(fl.get(2), Some(30));
    }

    #[test]
    fn get_else_test() {
        let mut fl: SinglyLinkedList<i32> = SinglyLinkedList::new();
        fl.push_back(10);
        fl.push_back(20);
        fl.push_back(30);

        assert_eq!(fl.get(3), None);
    }

    #[test]
    fn is_sorted_empty_list_test() {
        let fl: SinglyLinkedList<i32> = SinglyLinkedList::new();
        assert_eq!(fl.is_sorted(), true);
    }

    #[test]
    fn is_sorted_non_empty_list_test() {
        let mut fl: SinglyLinkedList<i32> = SinglyLinkedList::new();
        fl.push_back(5);
        fl.push_back(6);
        fl.push_back(1);

        assert_eq!(fl.is_sorted(), false);
    }

    #[test]
    fn is_sorted_list_test() {
        let mut fl: SinglyLinkedList<i32> = SinglyLinkedList::new();
        fl.push_back(10);
        fl.push_back(20);
        fl.push_back(30);

        assert_eq!(fl.is_sorted(), true);
    }

    #[test]
    fn pop_back_empty_list_test() {
        let mut fl: SinglyLinkedList<i32> = SinglyLinkedList::new();
        assert_eq!(fl.pop_back().unwrap_or_else(|_| -1), -1);
    }

    #[test]
    fn pop_back_test() {
        let mut fl: SinglyLinkedList<i32> = SinglyLinkedList::new();
        fl.push_back(5);
        fl.push_back(6);
        fl.push_back(1);

        assert_eq!(fl.pop_back().unwrap(), 1);
        assert_eq!(fl.len(), 2);
        assert_eq!(fl.pop_back().unwrap(), 6);
        assert_eq!(fl.len(), 1);
        assert_eq!(fl.pop_back().unwrap(), 5);
        assert_eq!(fl.len(), 0);
    }
}
