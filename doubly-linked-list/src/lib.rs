// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
use std::ptr::NonNull;
use std::marker::PhantomData;
mod pre_implemented;

#[derive(Debug)]
pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _bool: PhantomData<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;
struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    cur: Link<T>,
}

#[derive(Debug)]
pub struct Iter<'a, T> {
    front: Link<T>,
    len: usize,
    _marker: PhantomData<&'a T>,
}


impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _bool: PhantomData,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<T> {
        let cur = self.front.map(|node| node);
        Cursor {
            list: self,
            cur,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        let cur = self.back.map(|node| node);
        Cursor {
            list: self,
            cur,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { 
            front: self.front,
            len: self.len,
            _marker: PhantomData,
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.cur.map(|node| &mut (*node.as_ptr()).elem)
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            let next = if let Some(cur) = self.cur {
                (*cur.as_ptr()).back
            }
            else {
                None
            };
            self.cur = next;
            next.map(|node| &mut (*node.as_ptr()).elem)
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            let prev = if let Some(cur) = self.cur {
                (*cur.as_ptr()).front
            }
            else {
                None
            };
            self.cur = prev;
            prev.map(|node| &mut (*node.as_ptr()).elem)
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        if self.list.is_empty() {
            return None;
        }
        unsafe {
            match self.cur {
                Some(node) => {
                    match ((*node.as_ptr()).front, (*node.as_ptr()).back) {
                        (Some(f), Some(b)) => {
                            (*f.as_ptr()).back = Some(b);
                            (*b.as_ptr()).front = Some(f);
                            self.cur = Some(b);
                        },
                        (Some(f), None) => {
                            (*f.as_ptr()).back = None;
                            self.list.back = Some(f);
                            self.cur = Some(f);
                        },
                        (None, Some(b)) => {
                            (*b.as_ptr()).front = None;
                            self.list.front = Some(b);
                            self.cur = Some(b);
                        },
                        (None, None) => {
                            self.list.back = None;
                            self.list.front = None;
                            self.cur = None;
                        },
                    }
                    
                    self.list.len -= 1;
                    Some(Box::from_raw(node.as_ptr()).elem)
                },
                None => None,
            }
        }
    }

    pub fn insert_after(&mut self, _element: T) {
        unsafe {
            let new_ptr = NonNull::new_unchecked(Box::into_raw(
                Box::new(
                    Node {
                        front: None,
                        back: None,
                        elem: _element,
                    })));
            match self.cur {
                Some(node) => {
                    match (*node.as_ptr()).back {
                        Some(b) => {
                            (*new_ptr.as_ptr()).back = Some(b);
                            (*new_ptr.as_ptr()).front = Some(node);
                            (*node.as_ptr()).back = Some(new_ptr);
                            (*b.as_ptr()).front = Some(new_ptr);
                        },
                        None => {
                            (*new_ptr.as_ptr()).front = Some(node);
                            (*node.as_ptr()).back = Some(new_ptr);
                            self.list.back = Some(new_ptr);
                        },
                    }
                },
                None => {
                    self.list.front = Some(new_ptr);
                    self.list.back = Some(new_ptr);
                    self.cur = Some(new_ptr);
                },
            };
            self.list.len += 1;
        }
    }

    pub fn insert_before(&mut self, _element: T) {
        unsafe {
            let new_ptr = NonNull::new_unchecked(
                Box::into_raw(Box::new(
                        Node {
                            front: None,
                            back: None,
                            elem: _element,
                        })));
            match self.cur {
                Some(node) => {
                    match (*node.as_ptr()).front {
                        Some(f) => {
                            (*new_ptr.as_ptr()).front = Some(f);
                            (*new_ptr.as_ptr()).back = Some(node);
                            (*node.as_ptr()).front = Some(new_ptr);
                            (*f.as_ptr()).back = Some(new_ptr);
                        },
                        None => {
                            (*new_ptr.as_ptr()).back = Some(node);
                            (*node.as_ptr()).front = Some(new_ptr);
                            self.list.front = Some(new_ptr);
                        },
                    }
                },
                None => {
                    self.list.front = Some(new_ptr);
                    self.list.back = Some(new_ptr);
                    self.cur = Some(new_ptr);
                },
            };
            self.list.len += 1;
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.len -= 1;
                self.front = (*node.as_ptr()).back;
                &(*node.as_ptr()).elem
            })
        }
        else {
            None
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

unsafe impl<'a, T: Send> Send for Iter<'a, T> {}
unsafe impl<'a, T: Sync> Sync for Iter<'a, T> {}
