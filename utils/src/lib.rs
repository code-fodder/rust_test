
use std::fmt;

pub fn test123() {
    println!("test!");
}

// pub fn add(a: u64, b: u64) -> u64 {
//     return a + b;
// }

pub fn print_item<T>(a: T) where T: fmt::Display {
    println!("print_item: {}", a);
}


// pub fn add<A: std::ops::Add<Output = A>, B: std::ops::Add>(a: A, b: B) -> A
// {
//     return a + b;
// }

pub struct Adder<T>(std::marker::PhantomData<T>);
impl<T> Adder<T> {
    pub fn add<A, B>(a: A, b: B) -> T
      where T: std::ops::Add<Output = T>, A: Into<T>, B: Into<T> {
        return a.into() + b.into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Adder::<f64>::add(1, 2.3), 3.3);
        assert_eq!(Adder::<i64>::add(1, 2), 3);
        assert_ne!(Adder::<f64>::add(1, 2.3), 0.0);
    }
}
