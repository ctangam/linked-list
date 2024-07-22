use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl <T> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
            next: None,
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_tail = Box::new(Node::new(elem));
        
        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);


                self.tail = (*self.tail).next.as_deref_mut().unwrap() as *mut _;
            }
        } else {
            self.head = Some(new_tail);

            self.tail = self.head.as_deref_mut().unwrap() as *mut _;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
        
    }
}

