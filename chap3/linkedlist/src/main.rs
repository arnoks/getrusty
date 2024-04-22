
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

struct List {
    head: Option<Box<ListNode>>,
}
impl List {
    fn new() -> Self {
        List { head: None }
    }
    fn push(&mut self, val: i32) {
        let new_node = ListNode {
            val,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }
    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }
}

use crate::List1;
struct List1 {
    Cons(i32, Box<List1>),
    Nil,
}

impl List1 {
    fn new() -> Self {
        List1::Nil
    }
    fn push(&mut self, val: i32) {
        let new_node = List1::Cons(val, Box::new(List1::Nil));
        *self = List1::Cons(val, Box::new(List1::Nil));
    }
    fn pop(&mut self) -> Option<i32> {
        match self {
            List1::Cons(val, next) => {
                let val = *val;
                *self = *next;
                Some(val)
            }
            List1::Nil => None,
        }
    }
    fn len(&self) -> usize {
        match self {
            List1::Cons(_, next) => 1 + next.len(),
            List1::Nil => 0,
        }
    }
    fn stringify(&self) -> String {
        match self {
            List1::Cons(val, next) => {
                format!("{}, {}", val, next.stringify())
            }
            List1::Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
    let mut list1 = List1::new();
    list1.push(1);
    list1.push(2);
    list1.push(3);

    println!("{:?}", list1.stringify!());
}
