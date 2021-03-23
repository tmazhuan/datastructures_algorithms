use lists::dll::*;
fn main() {
    let mut l = DlList::new();
    for i in 1..6 {
        // println!("{}", i);
        assert_eq!(i, l.enqueue(i));
    }
    //current position should be the head
    //assert_eq!(1, l.get_current_position().unwrap());
    while l.move_forward() {}
    // while l.move_backward() {}
    //we should be at the tail now
    //lets move one backwards
    // l.move_backward();
    assert_eq!(5, l.get_current_position().unwrap());
    assert_eq!(4, l.get_tail().unwrap());
    assert_eq!(1, l.get_head().unwrap());
    while l.move_backward() {}
    assert_eq!(2, l.get_current_position().unwrap());
    assert_eq!(3, l.get_current_position().unwrap());
    assert!(l.get_current_position().is_none());
}
