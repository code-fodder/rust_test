
// EXTERN_C
extern  {
    fn cadd(a: libc::c_int, b: libc::c_int) -> libc::c_int;
}

fn check() -> bool {
    return true;
}

fn main() {

    println!("Hello, world!");

    utils::test123();

    utils::print_item(12);
    utils::print_item(10.32);
    utils::print_item("test");

    println!("add int int:   {}", utils::Adder::<i32>::add(1, 2));
    println!("add int float: {}", utils::Adder::<f64>::add(1, 2.3));

    println!("local check function: {}", check());


    // EXTERN_C
    println!("\nExtern C testing");
    unsafe {
        let res = cadd(3, 2);
        println!("result is {}", res);
    }
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