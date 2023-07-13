struct Item(String, u16);

fn main() {
    let banana = Item("banana".to_string(), 3000);
    let apple = Item("apple".to_string(), 500);

    let total = banana.1 + apple.1;
    print_tuple(banana);
    print_tuple(apple);

    println!("total: {}", total)
}

fn print_tuple(item: Item) {
    println!("{}は{}円", item.0, item.1)
}
