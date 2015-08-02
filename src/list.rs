use std::mem;

#[allow(dead_code)]
pub struct List<T> {
    head: Link<T>,
}

#[allow(dead_code)]
enum Link <T>{
    Empty,
    More(Box<Node<T>>),
}

#[allow(dead_code)]
struct Node <T> {
    elem: T,
    next: Link<T>,
}


impl<T> List<T>{
    pub fn new() -> Self {
        List{head : Link::Empty}
    }
   
    pub fn push(&mut self, elem : T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
       self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                None    
            }
            Link::More(boxed_node) => {
                let node = *boxed_node;
                self.head = node.next;
                Some(node.elem)                
            }
        }
    }
}

impl<T> Drop for List <T>{
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

#[test]
fn test_new() {
    let _list = List::<i32>::new();

    assert!(true);
}

#[test]
fn test_push() {
    let mut list = List::new();
    list.push(0);
    assert!(true);
}

#[test]
fn test_pop() {
    let mut list = List::new();
    list.push(1);
    assert_eq!(1, list.pop().unwrap());   
}

