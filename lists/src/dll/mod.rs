//! This module implements a double linked list with enqueue, a consuming get_head, a consuming get_tail,
//! and non consuming peek
//! For long queues we should implement the Drop function to ensure successful disposal of the queue.   
use std::cell::{Ref, RefCell};
use std::rc::Rc;

///Type to simplify variable declaration
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

///A single node in the queue with a value and a pointer to another node.
pub struct Node<T> {
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

    ///attaches a new Node to the end of its queue, moves the references of the list accordingly and returns its new length
    /// # Attributes
    /// * `value`- the value to append
    pub fn enqueue(&mut self, value: T) -> i32 {
        // DlList::print_pointer_count(self.head.as_ref(), "in Enqueue at beginning", "head");
        // DlList::print_pointer_count(self.tail.as_ref(), "in Enqueue at beginning", "tail");
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
        // DlList::print_pointer_count(self.head.as_ref(), "in Enqueue at after match", "head");
        // DlList::print_pointer_count(self.tail.as_ref(), "in Enqueue at after match", "tail");
        self.length += 1;
        self.length
    }

    ///Removes and returns the head item in the list and moves the references of the list accordingly
    pub fn get_head(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                //what is the next element in the list
                let next = head.borrow_mut().next.take();
                match next {
                    Some(next) => {
                        //we need to set the prevoius point to None as it is the head now
                        next.borrow_mut().previous = None;
                        //We need to check if current_position is pointing to our taken head
                        if Rc::ptr_eq(&self.current_position.as_ref().unwrap(), &head) {
                            //if it is we move the curren_position to our new head
                            self.current_position = Some(Rc::clone(&next));
                        }
                        //and we set the head
                        self.head = Some(next);
                    }
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

    ///Removes and returns the tail item in the list and moves the references of the list accordingly
    pub fn get_tail(&mut self) -> Option<T> {
        match self.tail.take() {
            Some(tail) => {
                //what is the previous element in the list
                let previous = tail.borrow_mut().previous.take();
                match previous {
                    Some(previous) => {
                        //we need to set the next point to None as it is the tail now
                        previous.borrow_mut().next = None;
                        //We need to check if current_position is pointing to our taken tail
                        if Rc::ptr_eq(&self.current_position.as_ref().unwrap(), &tail) {
                            //if it is we move the curren_position to our new tail
                            self.current_position = Some(Rc::clone(&previous));
                        }
                        self.tail = Some(previous);
                    }
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

    fn print_pointer_count(
        pointer: std::option::Option<&std::rc::Rc<std::cell::RefCell<Node<T>>>>,
        location: &str,
        pointer_name: &str,
    ) {
        match pointer {
            Some(p) => println!(
                "Pointer {} Count at {}: {}",
                pointer_name,
                location,
                Rc::strong_count(&p)
            ),
            None => println!("Pointer {} is NONE at {}", pointer_name, location),
        }
    }

    ///Removes and returns the item at the current position and advances the current Position in the list and moves the references of the list accordingly
    pub fn get_current_position(&mut self) -> Option<T> {
        DlList::print_pointer_count(
            self.current_position.as_ref(),
            "Beginning of get_current_position()",
            "current_position",
        );
        //get the current_position
        match self.current_position.take() {
            Some(cp) => {
                let previous_item = cp.borrow_mut().previous.take();
                let next_item = cp.borrow_mut().next.take();
                match previous_item {
                    Some(previous_item) => {
                        //we are not at the head
                        match next_item {
                            Some(next_item) => {
                                //current position was not at the head
                                //first we set the previous item next element to the next_item
                                previous_item.borrow_mut().next = Some(Rc::clone(&next_item));
                                //then we should set the next item previous element to the previous element
                                next_item.borrow_mut().previous = Some(Rc::clone(&previous_item));
                                //and we advance the currenposition to the next element
                                self.current_position = Some(Rc::clone(&next_item));
                            }
                            None => {
                                //current position was not at the head
                                //first we set the previous item next element to None as it is the new tail
                                previous_item.borrow_mut().next = None;
                                //current Position was at the tail
                                //we need to set the tail to the previous item
                                self.tail = Some(Rc::clone(&previous_item));
                                //we need to set the curren_position to the previous item
                                self.current_position = Some(Rc::clone(&previous_item));
                            }
                        }
                    }
                    None => {
                        //current position was at the head
                        match next_item {
                            Some(next_item) => {
                                //We set the next_item previous element to None as it is the new head
                                next_item.borrow_mut().previous = None;
                                //we need to set the curren_position to the next item
                                self.current_position = Some(Rc::clone(&next_item));
                                //and we set the head to the next item
                                self.head = Some(next_item);
                            }
                            None => {
                                //there is no item in the list after taking current-position
                                self.head = None;
                                self.tail = None;
                                self.current_position = None;
                            }
                        }
                    }
                }
                self.length -= 1;
                DlList::print_pointer_count(
                    self.current_position.as_ref(),
                    "In get_current_position before Rc::try_unwrap",
                    "current_position",
                );
                match Rc::try_unwrap(cp) {
                    Ok(i) => Some(i.into_inner().value),
                    Err(rc) => {
                        DlList::print_pointer_count(
                            self.current_position.as_ref(),
                            "In get_current_position in Rc::try_unwrap Error case",
                            "current_position",
                        );
                        panic!(
                            "Something is wrong. We shouldn't arrive here!!! Pointer Count: {}",
                            Rc::strong_count(&rc)
                        )
                    }
                }
            }
            //if it is None we should assert, that the list is empty and return None
            None => {
                assert_eq!(0, self.length);
                return None;
            }
        }
    }

    ///Moves the current position one item forward in the list and returns true if the move was successful, false otherwise.
    pub fn move_forward(&mut self) -> bool {
        DlList::print_pointer_count(self.head.as_ref(), "in move_forward at beginning", "head");
        let cp: Link<T>;
        if self.length == 0 {
            return false;
        } else {
            let c = self.current_position.as_ref().unwrap().borrow();
            if c.next.is_none() {
                return false;
            } else {
                let mut next = Some(Ref::map(c, |node| node.next.as_ref().unwrap()));
                cp = match next.take() {
                    Some(x) => Some(Rc::clone(&*x)),
                    None => return false,
                };
            }
        }
        self.current_position = cp;
        DlList::print_pointer_count(self.head.as_ref(), "in move_forward at end", "head");
        return true;
    }

    ///Moves the current position one item backwards in the list and returns true if the move was successful, false otherwise.
    pub fn move_backward(&mut self) -> bool {
        // DlList::print_pointer_count(self.head.as_ref(), "in move_backward at beginning", "head");
        let cp: Link<T>;
        if self.length == 0 {
            return false;
        } else {
            let c = self.current_position.as_ref().unwrap().borrow();
            if c.previous.is_none() {
                return false;
            } else {
                let mut previous = Some(Ref::map(c, |node| node.previous.as_ref().unwrap()));
                cp = match previous.take() {
                    Some(x) => Some(Rc::clone(&*x)),
                    None => return false,
                };
            }
        }
        self.current_position = cp;
        // DlList::print_pointer_count(self.head.as_ref(), "in move_backward at end", "head");
        return true;
    }

    ///Peeks the reference to the value of the current position without consuming it. The value is returned inside a `Ref`
    pub fn peek_current_position(&self) -> Option<Ref<T>> {
        self.current_position
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.value))
    }

    ///Peeks the reference to the value of the head position without consuming it. The value is returned inside a `Ref`
    pub fn peek_head(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.value))
    }

    ///Peeks the reference to the value of the tail position without consuming it. The value is returned inside a `Ref`
    pub fn peek_tail(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.value))
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

    #[test]
    fn test_multi_item_dll_head() {
        let mut l = DlList::new();
        assert_eq!(1, l.enqueue(String::from("test1")));
        assert_eq!(2, l.enqueue(String::from("test2")));
        assert_eq!(2, l.get_length());
        assert_eq!(String::from("test1"), l.get_head().unwrap());
        assert_eq!(l.get_length(), 1);
        assert_eq!(String::from("test2"), l.get_head().unwrap());
        assert_eq!(l.get_length(), 0);
        assert!(l.get_head().is_none());
    }

    #[test]
    fn test_multi_item_dll_tail() {
        let mut l = DlList::new();
        assert_eq!(1, l.enqueue(String::from("test1")));
        assert_eq!(2, l.enqueue(String::from("test2")));
        assert_eq!(2, l.get_length());
        assert_eq!(String::from("test2"), l.get_tail().unwrap());
        assert_eq!(l.get_length(), 1);
        assert_eq!(String::from("test1"), l.get_tail().unwrap());
        assert_eq!(l.get_length(), 0);
        assert!(l.get_tail().is_none());
    }
    #[test]
    fn test_multi_item_dll_mixed() {
        let mut l = DlList::new();
        assert_eq!(1, l.enqueue(String::from("test1"))); //
        assert_eq!(2, l.enqueue(String::from("test2"))); //
        assert_eq!(3, l.enqueue(String::from("test3")));
        assert_eq!(4, l.enqueue(String::from("test4")));
        assert_eq!(5, l.enqueue(String::from("test5"))); //
        assert_eq!(5, l.get_length());
        assert_eq!(String::from("test5"), l.get_tail().unwrap());
        assert_eq!(l.get_length(), 4);
        assert_eq!(String::from("test1"), l.get_head().unwrap());
        assert_eq!(l.get_length(), 3);
        assert_eq!(4, l.enqueue(String::from("test6"))); //
        assert_eq!(l.get_length(), 4);
        assert_eq!(String::from("test2"), l.get_head().unwrap());
        assert_eq!(String::from("test6"), l.get_tail().unwrap());
        assert_eq!(String::from("test4"), l.get_tail().unwrap());
        assert_eq!(String::from("test3"), l.get_head().unwrap());
        assert!(l.get_head().is_none());
        assert!(l.get_tail().is_none());
        assert_eq!(0, l.get_length());
    }

    #[test]
    fn test_peek() {
        let mut l = DlList::new();
        assert_eq!(1, l.enqueue(String::from("test1")));
        assert_eq!(2, l.enqueue(String::from("test2")));
        assert_eq!(String::from("test1"), *l.peek_head().unwrap());
        assert_eq!(String::from("test2"), *l.peek_tail().unwrap());
        assert_eq!(String::from("test1"), *l.peek_current_position().unwrap());
        assert_eq!(String::from("test1"), *l.peek_current_position().unwrap());
        assert!(l.move_forward());
        assert_eq!(String::from("test2"), *l.peek_current_position().unwrap());
        assert!(!l.move_forward());
        assert!(l.move_backward());
        assert_eq!(String::from("test1"), *l.peek_current_position().unwrap());
        assert!(!l.move_backward());
        assert_eq!(String::from("test1"), l.get_head().unwrap());
        assert_eq!(String::from("test2"), *l.peek_current_position().unwrap());
        assert_eq!(String::from("test2"), l.get_head().unwrap());
        assert!(l.peek_current_position().is_none());
    }

    #[test]
    fn test_get_current_position() {
        let mut l = DlList::new();
        for i in 1..5 {
            assert_eq!(i, l.enqueue(i));
        }
        //current position should be the head
        assert_eq!(1, l.get_current_position().unwrap());
        while l.move_forward() {}
        //we should be at the tail now
        //lets move one backwards
        l.move_backward();
        assert_eq!(3, l.get_current_position().unwrap());
    }
}
