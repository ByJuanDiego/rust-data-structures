#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            value: data,
            next: None,
        }
    }

    fn from(data: T, next: Option<Box<Self>>) -> Self {
        Self {
            value: data,
            next,
        }
    }
}

#[derive(Debug)]
pub struct ForwardList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> ForwardList<T>
    where T: PartialOrd {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let old_head = self.head.take();
        let new_head = Box::new(Node::from(data, old_head));

        self.head = Some(new_head);
        self.size += 1;
    }

    pub fn push_back(&mut self, data: T) {
        if self.is_empty() {
            self.head = Some(Box::new(Node::new(data)));
        } else {
            let mut current: &mut Option<Box<Node<T>>> = &mut self.head;
            while current.is_some() && current.as_ref().unwrap().next.is_some() {
                current = &mut current.as_mut().unwrap().next;
            }
            current.as_mut().unwrap().next = Some(Box::new(Node::new(data)));
        }

        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Result<T, &'static str> {
        if self.is_empty() {
            return Err("Can't pop an element from an empty ds-list");
        }

        let old_head = self.head.take().unwrap();

        self.head = old_head.next;
        self.size -= 1;

        Ok(old_head.value)
    }

    pub fn pop_back(&mut self) -> Result<T, &'static str> {
        if self.is_empty() {
            return Err("Can't pop an element from an empty ds-list");
        }

        if self.len() == 1 {
            let removed_value = self.head.take().unwrap().value;
            self.head = None;
            self.size = 0;
            return Ok(removed_value);
        }

        let mut current: &mut Option<Box<Node<T>>> = &mut self.head;

        while current.as_ref().unwrap().next.is_some() && current.as_ref().unwrap().next.as_ref().unwrap().next.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }

        let removed_value = current.as_mut().unwrap().next.take().unwrap().value;
        current.as_mut().unwrap().next = None;
        self.size -= 1;
        Ok(removed_value)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn is_sorted(&self) -> bool {
        let mut current: Option<&Box<Node<T>>> = self.head.as_ref();

        while current.is_some() && current.as_ref().unwrap().next.is_some() {
            if current.unwrap().value > current.as_ref().unwrap().next.as_ref().unwrap().value {
                return false;
            }
            current = current.unwrap().next.as_ref();
        }

        true
    }

    pub fn get(&mut self, index: usize) -> Result<&T, &'static str> {
        if index >= self.len() {
            return Err("Index out of range");
        }

        let mut current: &mut Option<Box<Node<T>>> = &mut self.head;

        for _ in 0..index {
            current = &mut current.as_mut().unwrap().next;
        }

        Ok(&current.as_ref().unwrap().value)
    }

    pub fn len(&self) -> usize {
        self.size
    }
}


#[cfg(test)]
mod tests {
    use super::ForwardList;

    #[test]
    fn empty_list_test() {
        let fl: ForwardList<i32> = ForwardList::new();
        assert_eq!(fl.len(), 0);
    }

    #[test]
    fn pop_front_in_empty_list_test() {
        let mut fl: ForwardList<i32> = ForwardList::new();
        assert!(fl.pop_front().is_err());
    }

    #[test]
    fn pop_front_in_non_empty_list_test() {
        let mut fl: ForwardList<i32> = ForwardList::new();
        fl.push_front(10);
        fl.push_front(20);
        fl.push_front(30);

        assert_eq!(fl.pop_front().unwrap(), 30);
        assert_eq!(fl.pop_front().unwrap(), 20);
        assert_eq!(fl.pop_front().unwrap(), 10);
    }


    #[test]
    fn push_back_test() {
        let mut fl: ForwardList<i32> = ForwardList::new();
        fl.push_back(10);
        fl.push_back(20);
        fl.push_back(30);

        assert_eq!(fl.len(), 3);
    }

    #[test]
    fn get_unwrap_test() {
        let mut fl: ForwardList<i32> = ForwardList::new();
        fl.push_back(10);
        fl.push_back(20);
        fl.push_back(30);

        assert_eq!(fl.get(0).unwrap_or_else(|_| &-1), &10);
        assert_eq!(fl.get(1).unwrap_or_else(|_| &-1), &20);
        assert_eq!(fl.get(2).unwrap_or_else(|_| &-1), &30);
    }

    #[test]
    fn get_else_test() {
        let mut fl: ForwardList<i32> = ForwardList::new();
        fl.push_back(10);
        fl.push_back(20);
        fl.push_back(30);

        assert_eq!(fl.get(3).unwrap_or_else(|_| &-1), &-1);
    }

    #[test]
    fn is_sorted_empty_list_test() {
        let fl: ForwardList<i32> = ForwardList::new();
        assert_eq!(fl.is_sorted(), true);
    }

    #[test]
    fn is_sorted_non_empty_list_test() {
        let mut fl: ForwardList<i32> = ForwardList::new();
        fl.push_back(5);
        fl.push_back(6);
        fl.push_back(1);

        assert_eq!(fl.is_sorted(), false);
    }

    #[test]
    fn is_sorted_list_test() {
        let mut fl: ForwardList<i32> = ForwardList::new();
        fl.push_back(10);
        fl.push_back(20);
        fl.push_back(30);

        assert_eq!(fl.is_sorted(), true);
    }

    #[test]
    fn pop_back_empty_list_test() {
        let mut fl: ForwardList<i32> = ForwardList::new();
        assert_eq!(fl.pop_back().unwrap_or_else(|_| -1), -1);
    }

    #[test]
    fn pop_back_test() {
        let mut fl: ForwardList<i32> = ForwardList::new();
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
