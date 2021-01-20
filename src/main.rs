fn main() {
    println!("Hello, world!");

    println!("adder: {:?}", utils::Adder::<i32>::add(3, 4));
    println!("adder: {:?}", utils::Adder::<i32>::add(5, 6));

    utils::test123();

    utils::print_item(12);
    utils::print_item("ertr");


    println!("add int int:   {}", utils::Adder::<i32>::add(1, 2));
    println!("add int float: {}", utils::Adder::<f64>::add(1, 2.3));

}
