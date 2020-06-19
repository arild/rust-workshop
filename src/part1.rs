#![allow(unused_variables)]

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn say_hello(name: &str) -> String {
    String::from("Hello ") + name
}

fn concat_strings(tuple: (&str, &str)) -> String {
    let (s1, s2) = tuple;

    s1.to_string() + s2
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(4, 3), 7);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(1, 2), 2);
        assert_eq!(max(4, 3), 4);
    }

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(3), false);
        assert_eq!(is_even(4), true);
    }

    #[test]
    fn test_say_hello() {
        assert_eq!(say_hello("John"), "Hello John");
    }

    #[test]
    fn test_concat_strings() {
        assert_eq!(concat_strings(("John", " Smith")), "John Smith");
    }
}
