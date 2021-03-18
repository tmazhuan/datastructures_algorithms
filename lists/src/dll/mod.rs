//! This module implements a double linked list with enqueue, a consuming get_head, a consuming get_tail,
//! and non consuming move_forward, move_backward
//! For long queues we should implement the Drop function to ensure successful disposal of the queue.   
use std::cell::RefCell;
use std::rc::Rc;

///Type to simplify variable declaration
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

///A single node in the queue with a value and a pointer to another node.
struct Node<T> {
    value: T,
    next: Link<T>,
    previous: Link<T>,
}

///The FIFO queue with a pointer to its head and tail
pub struct DlList<T> {
    head: Link<T>,
    tail: Link<T>,
    current_position: Link<T>,
    length: i32,
}

impl<T> Node<T> {
    ///Returns a new Node with the value `value`
    /// # Attributes
    /// * `value`- the value to assign to the node
    pub fn new(value: T) -> Node<T> {
        Node {
            value: value,
            next: None,
            previous: None,
        }
    }
}

impl<T> DlList<T> {
    ///Returns a new empty FIFO queue
    pub fn new() -> DlList<T> {
        DlList {
            head: None,
            tail: None,
            current_position: None,
            length: 0,
        }
    }
    ///Returns the length of its queue
    pub fn get_length(&self) -> i32 {
        self.length
    }

    ///attaches a new Node to the end of its queue and returns its new length
    /// # Attributes
    /// * `value`- the value to append
    pub fn enqueue(&mut self, value: T) -> i32 {
        let mut new_node = Node::new(value);
        //lets take the tail and match it to see if we enter the first item
        match self.tail.take() {
            Some(old_tail) => {
                new_node.previous = Some(Rc::clone(&old_tail));
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
                let ref_node_current_pos = Rc::clone(&ref_node_head);
                self.head = Some(ref_node_head);
                //... and tail
                self.tail = Some(ref_node_tail);
                //...and current current_position
                self.current_position = Some(ref_node_current_pos);
            }
        }
        self.length += 1;
        self.length
    }

    ///Removes and returns the head item in the list
    pub fn get_head(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                //what is the next element in the list
                let next = head.borrow_mut().next.take();
                match next {
                    Some(next) => self.head = Some(next),
                    None => {
                        //no item in the list anymore
                        self.tail = None;
                        self.current_position = None;
                    }
                };
                self.length -= 1;
                match Rc::try_unwrap(head) {
                    Ok(i) => Some(i.into_inner().value),
                    Err(_) => panic!("Something is wrong. We shouldn't arrive here!!!"),
                }
            }
            None => None,
        }
    }

    ///Removes and returns the tail item in the list
    pub fn get_tail(&mut self) -> Option<T> {
        match self.tail.take() {
            Some(tail) => {
                //what is the next element in the list
                let previous = tail.borrow_mut().previous.take();
                match previous {
                    Some(previous) => self.tail = Some(previous),
                    None => {
                        //no item in the list anymore
                        self.head = None;
                        self.current_position = None;
                    }
                };
                self.length -= 1;
                match Rc::try_unwrap(tail) {
                    Ok(i) => Some(i.into_inner().value),
                    Err(_) => panic!("Something is wrong. We shouldn't arrive here!!!"),
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_dll() {
        let l: DlList<i32> = DlList::new();
        assert_eq!(0, l.get_length());
    }
    #[test]
    fn test_empty_dll_head() {
        let mut l: DlList<i32> = DlList::new();
        assert!(l.get_head().is_none());
    }

    #[test]
    fn test_empty_dll_tail() {
        let mut l: DlList<i32> = DlList::new();
        assert!(l.get_tail().is_none());
    }
    #[test]
    fn test_one_item_dll_head() {
        let mut l = DlList::new();
        assert_eq!(1, l.enqueue(String::from("test")));
        assert_eq!(l.get_length(), 1);
        assert_eq!(String::from("test"), l.get_head().unwrap());
        assert_eq!(l.get_length(), 0);
    }

    #[test]
    fn test_one_item_dll_tail() {
        let mut l = DlList::new();
        assert_eq!(1, l.enqueue(String::from("test")));
        assert_eq!(l.get_length(), 1);
        assert_eq!(String::from("test"), l.get_tail().unwrap());
        assert_eq!(l.get_length(), 0);
    }

    // #[test]
    // fn test_multi_item_lifo() {
    //     let mut l = DlList::new();
    //     assert_eq!(1, l.enqueue(1));
    //     assert_eq!(2, l.enqueue(2));
    //     assert_eq!(3, l.enqueue(3));
    //     assert_eq!(4, l.enqueue(4));
    //     assert_eq!(5, l.enqueue(5));
    //     let i = 5;
    //     assert_eq!(l.get_length(), i);
    //     assert_eq!(5, l.get_tail().unwrap());
    //     assert_eq!(l.get_length(), i - 1);
    //     //assert_eq!(1, l.get_head().unwrap());
    //     // assert!(l.head.is_none());
    //     assert_eq!(l.get_length(), i - 2);
    // }
}
