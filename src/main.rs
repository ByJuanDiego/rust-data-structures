use ds_list::{List, SinglyLinkedList as FL, DoublyLinkedList as DL};

fn main() {
    let mut dl: DL<i32> = DL::new();
    let mut fl: FL<i32> = FL::new();

    dl.push_front(1);
    dl.push_front(2);
    fl.push_front(1);
    fl.push_front(2);

    println!("{} --> {} --> NULL", fl.get(0).unwrap(), fl.get(1).unwrap());
    println!("=> SENTINEL <==> {} <==> {} <=", dl.get(0).unwrap(), dl.get(1).unwrap());
}
