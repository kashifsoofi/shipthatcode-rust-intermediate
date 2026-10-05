use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let counter = Rc::new(RefCell::new(0));
    let a = Rc::clone(&counter);
    let b = Rc::clone(&counter);

    // TODO: add 1 to the shared value through `a`, then again through `b`.
    // borrow_mut() hands you a guard, not the i32 - you have to write
    // THROUGH the guard for the change to land in the RefCell.
    *a.borrow_mut() += 1;
    *b.borrow_mut() += 1;

    println!("{}", counter.borrow());
}
