// use crate::List::*;

// State of my linked list
// A new Linked list does not contan any elements
// It is represented by a Nil. Nil is a node that signifies the end of the linked list.
// A Cons is a tuple struct that wraps an element and a pointer to the next node.
// When inserting the first Element into the list, the cons variant keeps the value and a pointer to the Nil node.
// When inserting the second element, the cons variant keeps the value and a pointer to the previous cons node.

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        List::Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        // Box::new wraps the tail of the list, which is the previous list
        List::Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail.
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // recursive calculation of the length.
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            List::Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            List::Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            }
            List::Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();
    println!("linked list has length: {}", list.len());
    println!("linked list: {}", list.stringify());
    // Prepend some elements
    list = list.prepend(1);
    println!("linked list has length: {}", list.len());
    println!("linked list: {}", list.stringify());
    list = list.prepend(2);
    println!("linked list has length: {}", list.len());
    println!("linked list: {}", list.stringify());
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
