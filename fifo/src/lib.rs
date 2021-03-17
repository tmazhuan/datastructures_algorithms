use std::cell::RefCell;
use std::rc::Rc;

struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

struct Fifo<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    length: i32,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value: value,
            next: None,
        }
    }
}

impl<T> Fifo<T> {
    pub fn new() -> Fifo<T> {
        Fifo {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn enqueue(&mut self, value: T) -> i32
    where
        T: Copy,
    {
        let new_node = Node::new(value);
        //lets take the tail and match it to see if we enter the first item
        match self.tail.take() {
            Some(old_tail) => {
                //we need to change the value so we borrow it mutables and set the next value to the new node
                let new_tail_ref = Rc::new(RefCell::new(new_node));
                let next_node_ref = Rc::clone(&new_tail_ref);
                old_tail.borrow_mut().next = Some(next_node_ref);
                self.tail = Some(new_tail_ref);
            }
            None => {
                //first item in the list
                //lets add it to the head...
                let ref_node_head = Rc::new(RefCell::new(new_node));
                let ref_node_tail = Rc::clone(&ref_node_head);
                self.head = Some(ref_node_head);
                //... and tail
                self.tail = Some(ref_node_tail);
            }
        }
        self.length += 1;
        self.length
    }

    pub fn dequeue(&mut self) -> Option<T>
    where
        T: Copy,
    {
        match self.head.take() {
            Some(head) => {
                //what is the next element in the list
                let next = head.borrow_mut().next.take();
                match next {
                    Some(next) => self.head = Some(next),
                    None => self.tail = None,
                };
                self.length -= 1;
                Some(head.borrow().value)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_node() {
        let n = Node::new("test");
        assert_eq!("test", n.value);
        assert!(n.next.is_none());
    }

    #[test]
    fn test_new_lifo() {
        let l: Fifo<i32> = Fifo::new();
        assert_eq!(0, l.length);
        assert!(l.head.is_none());
        assert!(l.tail.is_none());
    }
    #[test]
    fn test_one_item_lifo() {
        let mut l = Fifo::new();
        assert_eq!(1, l.enqueue("test"));
        assert_eq!(l.length, 1);
        assert_eq!("test", l.dequeue().unwrap());
        assert!(l.head.is_none());
        assert_eq!(l.length, 0);
    }

    #[test]
    fn test_multi_item_lifo() {
        let mut l = Fifo::new();
        assert_eq!(1, l.enqueue(1));
        assert_eq!(2, l.enqueue(2));
        assert_eq!(3, l.enqueue(3));
        assert_eq!(4, l.enqueue(4));
        assert_eq!(5, l.enqueue(5));
        let i = 5;
        assert_eq!(l.length, i);
        assert_eq!(1, l.dequeue().unwrap());
        assert!(l.head.is_some());
        assert_eq!(l.length, i - 1);
        assert_eq!(2, l.dequeue().unwrap());
        let tail = match l.tail {
            Some(tail) => Some(tail.borrow().value),
            None => None,
        };
        assert_eq!(Some(5), tail);
        // assert!(l.head.is_none());
        assert_eq!(l.length, i - 2);
    }
}
