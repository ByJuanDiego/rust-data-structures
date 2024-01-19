use forward_list::ForwardList;

fn main() {
    let mut fl: ForwardList<i32> = ForwardList::new();
    match fl.pop_front() {
        Ok(value) => {
            println!("value: {value}");
        }
        Err(what) => {
            println!("{what}");
        }
    }
}
