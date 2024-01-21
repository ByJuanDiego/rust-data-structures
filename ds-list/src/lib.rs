pub trait List<T: PartialOrd> {
    fn new() -> Self;
    fn push_front(&mut self, data: T);
    fn push_back(&mut self, data: T);
    fn pop_front(&mut self) -> Result<T, &'static str>;
    fn pop_back(&mut self) -> Result<T, &'static str>;
    fn is_empty(&self) -> bool;
    fn is_sorted(&self) -> bool;
    fn get(&mut self, index: usize) -> Option<&T>;
    fn len(&self) -> usize;
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn create(data: T) -> Self {
        Self {
            value: data,
            next: None,
        }
    }

    fn create_and_link(data: T, next: Option<Box<Self>>) -> Self {
        Self {
            value: data,
            next,
        }
    }
}

#[derive(Debug)]
pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: PartialOrd> List<T> for SinglyLinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    fn push_front(&mut self, data: T) {
        let old_head = self.head.take();
        let new_head = Box::new(Node::create_and_link(data, old_head));

        self.head = Some(new_head);
        self.size += 1;
    }

    fn push_back(&mut self, data: T) {
        if self.is_empty() {
            self.head = Some(Box::new(Node::create(data)));
        } else {
            let mut current: &mut Option<Box<Node<T>>> = &mut self.head;
            while current.is_some() && current.as_ref().unwrap().next.is_some() {
                current = &mut current.as_mut().unwrap().next;
            }
            current.as_mut().unwrap().next = Some(Box::new(Node::create(data)));
        }

        self.size += 1;
    }

    fn pop_front(&mut self) -> Result<T, &'static str> {
        if self.is_empty() {
            return Err("Can't pop an element from an empty ds-list");
        }

        let old_head = self.head.take().unwrap();

        self.head = old_head.next;
        self.size -= 1;

        Ok(old_head.value)
    }

    fn pop_back(&mut self) -> Result<T, &'static str> {
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

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn is_sorted(&self) -> bool {
        let mut current: Option<&Box<Node<T>>> = self.head.as_ref();

        while current.is_some() && current.as_ref().unwrap().next.is_some() {
            if current.unwrap().value > current.as_ref().unwrap().next.as_ref().unwrap().value {
                return false;
            }
            current = current.unwrap().next.as_ref();
        }

        true
    }

    fn get(&mut self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }

        let mut current: &mut Option<Box<Node<T>>> = &mut self.head;

        for _ in 0..index {
            current = &mut current.as_mut().unwrap().next;
        }

        Some(&current.as_ref().unwrap().value)
    }

    fn len(&self) -> usize {
        self.size
    }
}
