use ds_list::{List, SinglyLinkedList as FL};

fn main() {
    let mut fl: FL<i32> = FL::new();
    match fl.pop_front() {
        Ok(value) => {
            println!("value: {value}");
        }
        Err(what) => {
            println!("{what}");
        }
    }
}
