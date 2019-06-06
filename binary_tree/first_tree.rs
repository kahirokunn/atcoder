use std::collections::VecDeque;

#[derive(Clone)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn insert(&mut self, value: i32) {
        match &value < &self.value {
            true => match &mut self.left {
                Some(node) => node.insert(value),
                None => {
                    self.left.get_or_insert(self.gen_node(value));
                }
            },
            false => match &mut self.right {
                Some(node) => node.insert(value),
                None => {
                    self.right.get_or_insert(self.gen_node(value));
                }
            },
        };
    }

    fn gen_node(&self, value: i32) -> Box<Node> {
        Box::new(Node {
            value,
            left: None,
            right: None,
        })
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut list: Vec<i32> = vec![];
        let mut queue: VecDeque<Box<Node>> = VecDeque::new();
        queue.push_back(Box::new(self.clone()));

        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();
            list.push(node.value);
            if let Some(left_node) = node.left {
                queue.push_back(left_node);
            }
            if let Some(right_node) = node.right {
                queue.push_back(right_node);
            }
        }
        list
    }

    fn is_include(&self, value: i32) -> bool {
        let mut count = 0;
        let mut queue: VecDeque<Box<Node>> = VecDeque::new();
        queue.push_back(Box::new(self.clone()));

        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();
            count += 1;
            if node.value == value {
                println!("searched num: {}", count);
                return true;
            }

            if let Some(left_node) = node.left {
                queue.push_back(left_node);
            }
            if let Some(right_node) = node.right {
                queue.push_back(right_node);
            }
        }
        println!("searched num: {}", count);
        false
    }

    fn len(&self) -> usize {
        let mut count = 0;
        let mut queue: VecDeque<Box<Node>> = VecDeque::new();
        queue.push_back(Box::new(self.clone()));

        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();
            count += 1;

            if let Some(left_node) = node.left {
                queue.push_back(left_node);
            }
            if let Some(right_node) = node.right {
                queue.push_back(right_node);
            }
        }
        count
    }
}

fn main() {
    let mut binary_tree = Node {
        value: 2,
        left: None,
        right: None,
    };
    binary_tree.insert(1);
    binary_tree.insert(3);
    println!("len {}", binary_tree.len());
    println!("{:?}", binary_tree.to_vec());
    println!("{}", binary_tree.is_include(2));
    println!("{}", binary_tree.is_include(1));
    println!("{}", binary_tree.is_include(3));
    println!("{}", binary_tree.is_include(4));
}
