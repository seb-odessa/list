
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node <T> {
    elem: T,
    next: Link<T>,
}


impl<T> List<T>{
    pub fn new() -> Self {
        List{head : None}
    }
   
    pub fn push(&mut self, elem : T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
       self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
                let item = *node;
                self.head = item.next;
                item.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| { &node.elem })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| { &mut node.elem })
    }
}


#[test]
fn test_new() {
    let list = List::<i32>::new();
    assert!(list.head.is_none());
}

#[test]
fn test_push() {
    let mut list = List::new();
    list.push(0);
    assert_eq!(list.head.unwrap().elem, 0);
    
}

#[test]
fn test_peek() {
    let mut list = List::new();
    list.push(0);
    assert_eq!(*list.peek().unwrap(), 0);
    assert_eq!(*list.peek().unwrap(), 0);
    assert_eq!(*list.peek().unwrap(), 0);
   
}

#[test]
fn test_peek_mut() {
    let mut list = List::new();
    list.push(0);
    assert_eq!(*list.peek().unwrap(), 0);   
   *list.peek_mut().unwrap() = 1;
    assert_eq!(*list.peek().unwrap(), 1);
}


#[test]
fn test_pop() {
    let mut list = List::new();
    list.push(1);
    assert_eq!(1, list.pop().unwrap());   
}

#[test]
fn test_order() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(3, list.pop().unwrap());
    assert_eq!(2, list.pop().unwrap());
    assert_eq!(1, list.pop().unwrap());
}
