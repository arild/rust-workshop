#![allow(unused_variables, unreachable_code, dead_code)]

// Uncomment the unimplemented() macro and follow the hints in the 'TASK' comments
// Try compiling after removing unimplemented(). The compiler comes with good explanations.

// TASK 1: Allow for mutability
fn mutability() -> String {
    let mut text = String::from("this is a ");

    text.push_str("mutable string");

    text
}

// TASK 2: Avoid ownership violation
fn borrow_after_move() -> String {
    // The following code will make a copy of x in y and store both on the stack. This can be done
    // since they both have a known size at compile time.
    let x: i32 = 5;
    let y: i32 = x;

    // Both x and y can be borrowed here without any issues
    println!("x = {}, y = {}", x, y);

    // We can not do this with Strings, since they are stored on the heap.
    // When assigning s1 to s2 the data is moved to s2. Using s1 after moving it
    // will result in a compilation error
    let s1 = String::from("hello");
    let s2 = s1.clone();

    format!("text = {}, text2 = {}", s1, s2)
}

// TASK 3: Send reference to function instead of moving ownership
fn ownership_functions() -> String {
    // Passing a variable to a functions also moves it.
    // Change calculate_length so that it takes in a reference to s string instead, and
    // pass in a reference to 'text'. This will keep the ownership of text from being moved.
    let text: String = String::from("this prefix");
    let text_length = calculate_length(&text);

    format!("{} is of length {}", text, text_length)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// TASK 4: Invoke append_string with a mutable reference.
// PS: Don't do this in real life
fn mutable_reference() -> String {
    let mut text = String::from("this is a ");
    append_string(&mut text);
    text
}

fn append_string(s: &mut String) {
    s.push_str("mutable string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutability() {
        assert_eq!(mutability(), "this is a mutable string");
    }

    #[test]
    fn test_borrow_after_move() {
        assert_eq!(borrow_after_move(), "text = hello, text2 = hello");
    }

    #[test]
    fn test_ownership_functions() {
        assert_eq!(ownership_functions(), "this prefix is of length 11");
    }

    #[test]
    fn test_mutable_reference() {
        assert_eq!(mutable_reference(), "this is a mutable string");
    }
}
