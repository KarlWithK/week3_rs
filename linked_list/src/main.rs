pub mod linked_list;
use linked_list::ComputeNorm;
use linked_list::LinkedList;

fn main() {
    let mut list_one: LinkedList<char> = LinkedList::new();
    let mut list_two: LinkedList<char> = LinkedList::new();
    assert!(list_one.is_empty());
    assert_eq!(list_one.get_size(), 0);
    let word = String::from("Testing this works");
    word.chars().rev().for_each(|c| {
        list_one.push_front(c);
    });
    word.chars().rev().for_each(|c| list_two.push_front(c));
    assert!(list_one == list_two);
    println!("{}", list_one);
    println!("list size: {}", list_one.get_size());
    println!("top element: {}", list_one.pop_front().unwrap());
    println!("{}", list_one);
    println!("size: {}", list_one.get_size());
    println!("{}", list_one.to_string()); // ToString impl for anything impl Display

    (list_one).into_iter().for_each(|val| {
        println!("{}", val);
    });

    let mut norm: LinkedList<f64> = LinkedList::new();
    (0..16).for_each(|double| {
        norm.push_front(double as f64);
    });
    println!("{}", norm.compute_norm())
}
