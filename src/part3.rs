#![allow(unused_variables)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    unimplemented!()
}

fn get_value_or_zero(x: Option<i32>) -> i32 {
    unimplemented!()
}

fn subtract_one(x: Option<i32>) -> Option<i32> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_in_cents() {
        assert_eq!(value_in_cents(Coin::Penny), 1);
        assert_eq!(value_in_cents(Coin::Nickel), 5);
        assert_eq!(value_in_cents(Coin::Dime), 10);
        assert_eq!(value_in_cents(Coin::Quarter), 25);
    }

    #[test]
    fn test_get_value_or_zero() {
        assert_eq!(get_value_or_zero(Some(3)), 3);
        assert_eq!(get_value_or_zero(Some(0)), 0);
        assert_eq!(get_value_or_zero(None), 0);
    }

    #[test]
    fn test_subtract_one() {
        assert_eq!(subtract_one(Some(3)), Some(2));
        assert_eq!(subtract_one(Some(-1)), Some(-2));
        assert_eq!(subtract_one(None), None);
    }
}
