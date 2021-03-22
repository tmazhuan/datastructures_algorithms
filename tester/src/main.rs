use lists::dll::*;
fn main() {
    let mut dlist = DlList::new();
    for i in 0..1000 {
        dlist.enqueue(i);
    }
    let mut has_next_to_peek = dlist.get_length() > 0;
    while has_next_to_peek {
        println!("Peeking: {}", *dlist.peek_current_position().unwrap());
        has_next_to_peek = dlist.move_forward();
    }

    while let Some(x) = dlist.get_tail() {
        println!("getting: {}", x);
    }
    let mut has_next_to_peek = dlist.get_length() > 0;
    while has_next_to_peek {
        println!("Peeking: {}", *dlist.peek_current_position().unwrap());
        has_next_to_peek = dlist.move_forward();
    }
}
