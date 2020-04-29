use linked_list::first::List;

fn main() {
    println!("Hello, World!");

    let mut list = List::new();
    list.push(2);
    list.push(3);

    println!("{:?}", list);
}
