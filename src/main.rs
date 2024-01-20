use ds_list::ForwardList as List;

fn main() {
    let mut fl: List<i32> = List::new();
    match fl.pop_front() {
        Ok(value) => {
            println!("value: {value}");
        }
        Err(what) => {
            println!("{what}");
        }
    }
}
