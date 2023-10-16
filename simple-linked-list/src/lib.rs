use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
} 

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Self { data: data, next: next }
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, length: 0 }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node::new(_element, self.head.take())));
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_some() {
            let node = self.head.take().unwrap();
            self.head = node.next;
            self.length -= 1;
            return Some(node.data)
        }
        else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|h| &(h.data))
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut r = SimpleLinkedList::new();
        while let Some(x) = self.pop() {
            r.push(x);
        }
        r
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut it = SimpleLinkedList::new();
        for i in _iter {
            it.push(i);
        }
        it
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut v: Vec<T> = Vec::new();
        while let Some(x) = _linked_list.pop() {
            v.insert(0, x);
        }
        v
    }
}
