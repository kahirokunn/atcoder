// 全部同じpointerを指す

#[derive(Debug)]
struct Node<T> {
    elem: T,
}

fn test<T>(c: &Node<T>) {
    println!("{:p}", c);
}

fn main() {
    let a = Node { elem: 1 };
    let b = &a;
    println!("{:p}", &a);
    println!("{:p}", b);
    test(&a);
}
