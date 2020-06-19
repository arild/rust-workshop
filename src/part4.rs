#![allow(unused_variables)]

fn compute_sum_of_numbers(numbers: Vec<i32>) -> i32 {
    numbers.iter().sum()
}

fn find_even_numbers(numbers: Vec<i32>) -> Vec<i32> {
    numbers.into_iter().filter(|n| n % 2 == 0).collect()
}

fn find_positive_numbers(numbers: Vec<i32>) -> Vec<i32> {
    numbers.into_iter().filter(|n| *n > 0).collect()
}

fn find_last_number_or_zero(numbers: Vec<i32>) -> i32 {
    match numbers.last() {
        None => 0,
        Some(number) => *number,
    }
}

fn find_total_age(persons: Vec<Person>) -> i32 {
    persons.iter().fold(0, |acc, person| acc + person.age)
}

fn find_sub_list_from_predicate(
    persons: Vec<Person>,
    predicate: fn(&Person) -> bool,
) -> Vec<Person> {
    persons.into_iter().filter(|p| predicate(p)).collect()
}

// Duplicates elements in vector
// Example: [1, 5, 2] -> [1, 1, 5, 5, 2, 2]
fn duplicate(elements: Vec<i32>) -> Vec<i32> {
    elements.into_iter().map(|x| vec![x, x]).flatten().collect()
}

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_sum_of_numbers() {
        assert_eq!(compute_sum_of_numbers(vec![1, 2, 3]), 6);
        assert_eq!(compute_sum_of_numbers(vec![]), 0)
    }

    #[test]
    fn test_find_even_numbers() {
        assert_eq!(find_even_numbers(vec![1, 2, 3, 4]), vec![2, 4])
    }

    #[test]
    fn test_find_positive_numbers() {
        assert_eq!(
            find_positive_numbers(vec![1, -4, -8, 11, -200, -1, 8]),
            vec![1, 11, 8]
        );
        assert_eq!(find_positive_numbers(vec![0]), vec![])
    }

    #[test]
    fn test_find_last_item_or_zero() {
        assert_eq!(find_last_number_or_zero(vec![1, 2, 3]), 3);
        assert_eq!(find_last_number_or_zero(vec![]), 0);
    }

    #[test]
    fn test_find_total_age() {
        assert_eq!(
            find_total_age(vec![
                Person {
                    name: "John Smith".to_owned(),
                    age: 25
                },
                Person {
                    name: "Sandra White".to_owned(),
                    age: 19
                },
                Person {
                    name: "Paul Wright".to_owned(),
                    age: 64
                }
            ]),
            108
        );
    }

    #[test]
    fn test_find_sub_list_from_predicate() {
        let persons = vec![
            Person {
                name: "John Smith".to_owned(),
                age: 25,
            },
            Person {
                name: "Sandra White".to_owned(),
                age: 19,
            },
            Person {
                name: "Paul Wright".to_owned(),
                age: 64,
            },
        ];
        assert_eq!(
            find_sub_list_from_predicate(persons, over_age_30),
            vec![Person {
                name: "Paul Wright".to_owned(),
                age: 64
            }]
        );
    }

    #[test]
    fn test_duplicate() {
        assert_eq!(
            duplicate(vec![5, 1, 10, 11, 0]),
            vec![5, 5, 1, 1, 10, 10, 11, 11, 0, 0]
        );
        assert_eq!(duplicate(vec![]), vec![]);
    }

    fn over_age_30(person: &Person) -> bool {
        person.age > 30
    }
}
