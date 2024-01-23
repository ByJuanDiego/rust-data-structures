use ds_list::{List, DoublyLinkedList};

#[cfg(test)]
mod tests_doubly_linked_list {
    use super::*;

    #[test]
    fn empty_list_test() {
        let dl: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert_eq!(dl.len(), 0);
    }

    #[test]
    fn push_front_test() {
        let mut dl: DoublyLinkedList<i32> = DoublyLinkedList::new();
        dl.push_front(1);
        dl.push_front(2);
        dl.push_front(3);
        dl.push_front(4);
        dl.push_front(5);

        assert_eq!(dl.len(), 5);
    }

    #[test]
    fn get_empty_list_test() {
        let mut dl: DoublyLinkedList<i32> = DoublyLinkedList::new();

        assert_eq!(dl.get(0), None);
    }

    #[test]
    fn get_non_empty_list_test() {
        let mut dl: DoublyLinkedList<i32> = DoublyLinkedList::new();
        dl.push_back(1);
        dl.push_front(2);
        dl.push_back(3);
        dl.push_front(4);
        dl.push_back(5);
        dl.push_front(6);

        assert_eq!(dl.get(0), Some(6));
        assert_eq!(dl.get(1), Some(4));
        assert_eq!(dl.get(2), Some(2));
        assert_eq!(dl.get(3), Some(1));
        assert_eq!(dl.get(4), Some(3));
        assert_eq!(dl.get(5), Some(5));
    }
}
