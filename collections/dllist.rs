use std::boxed::Box;
use std::ptr::NonNull;

type MaybeNode<T> = Option<NonNull<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: MaybeNode<T>,
}

#[derive(Debug)]
struct SLList<T> {
    head: MaybeNode<T>,
    tail: MaybeNode<T>,
    count: u32,
}

impl<T> SLList<T> {
    pub fn new() -> Self {
        SLList {
            head: None,
            tail: None,
            count: 0,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let node = Node {
            elem,
            next: self.head,
        };
        self.head = self.node_to_maybe_node(node);
        if self.tail == None {
            self.tail = self.head
        }
        self.count += 1;
    }

    pub fn push_back(&mut self, elem: T) {
        self.push_back_node(Node { elem, next: None });
    }

    pub fn push_back_node(&mut self, node: Node<T>) {
        let new_tail_node = self.node_to_maybe_node(node);
        match self.tail {
            Some(tail) => {
                unsafe {
                    (*tail.as_ptr()).next = new_tail_node;
                }
                self.tail = new_tail_node;
            }
            None => {
                self.tail = new_tail_node;
            }
        }

        if self.head == None {
            self.head = self.tail;
        }
        self.count += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|head| {
            self.count -= 1;
            let head = self.non_null_to_box(head);
            self.head = head.next;
            match self.head {
                None => {
                    self.tail = None;
                }
                Some(head_next) => self.head = Some(head_next),
            }
            (*head).elem
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|tail| {
            self.count -= 1;
            let tail = self.non_null_to_box(tail);

            unsafe {
                let head = self.head.unwrap();
                let mut node = head;
                for _ in 0..self.count - 1 {
                    node = (*node.as_ptr()).next.unwrap();
                }
                (*node.as_ptr()).next = None;
                self.tail = Some(node);
            }
            (*tail).elem
        })
    }

    fn non_null_to_box(&self, non_null: NonNull<Node<T>>) -> Box<Node<T>> {
        unsafe { Box::from_raw(non_null.as_ptr()) }
    }

    fn node_to_maybe_node(&self, node: Node<T>) -> MaybeNode<T> {
        Some(unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(node))) })
    }
}

fn main() {
    let mut list = SLList::new();
    list.push_back(123);
    list.push_front(456);
    list.push_back(223);

    println!("{:?}", list.pop_front());
    println!("{:?}", list.pop_back());
    println!("{:?}", list.pop_front());
    println!("{:?}", list.pop_back());
    println!("{:?}", list.pop_front());
}
