// EXTERN_C
extern crate libc;

// EXTERN_C
extern  {
    fn cadd(a: libc::c_int, b: libc::c_int) -> libc::c_int;
}

fn check() -> bool {
    return true;
}

fn main() {
    // EXTERN_C
    unsafe {
        println!("C-Adding: {}", cadd(3, 2));
    }

    println!("Hello, world!");

    println!("adder: {:?}", utils::Adder::<i32>::add(3, 4));
    println!("adder: {:?}", utils::Adder::<i32>::add(5, 6));

    utils::test123();

    utils::print_item(12);
    utils::print_item("ertr");



    println!("add int int:   {}", utils::Adder::<i32>::add(1, 2));
    println!("add int float: {}", utils::Adder::<f64>::add(1, 2.3));
    println!("check: {}", check());

}

#[cfg(test)]
mod main_tester {
    use super::*;

    #[test]
    fn main_t1() {
        assert_eq!(check(), true);
    }

    #[test]
    fn main_t2() {
        assert_ne!(check(), false);
    }

    // EXTERN_C
    #[test]
    fn c_test() {
        unsafe {
            assert_eq!(cadd(3, 2), 5);
        }
    }
}