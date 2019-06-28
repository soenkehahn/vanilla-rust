fn main() {
    println!("hello world");
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod add {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(1, 1), 2)
    }
}
