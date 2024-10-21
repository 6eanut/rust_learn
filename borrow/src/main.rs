fn main() {
    let mut items = 5;
    print_item(&items);
    sub_item(&mut items);
    print_item(&items);
}

fn sub_item(item: &mut i32) {
    *item -= 1;
}

fn print_item(item: &i32) {
    println!("items = {}", item);
}
