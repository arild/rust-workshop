class: center, middle, inverse, title-slide

background-image: url(img/Tech_Rust.svg)
background-position: 85% 50%
background-size: 500pt

# Rust
## Introductory workshop
### Arild & Eirik - FINN

---

## Code: https://github.com/arild/rust-workshop
## Slides: https://arild.github.io/rust-workshop/
## Techday program: http://tech-day.finntech.no

---

# Outline
- Part 1: Fundamentals

- Part 2: Ownership

- Part 3: Pattern Matching & Option

- Part 4: Collections (Vectors)

- Part 5: Concurrency

---

background-image: url(img/most-loved-languages-2019.png)
background-position: 90% 50%
background-size: 450pt

# Rust
- Focus on speed & memory safety

- System level language

- Zero cost abstractions

- Fearless concurrency

---


# Hello World

```
fn print_number(number: i32) {
    println!("The number is: {}", number);
}

fn main() {
    let number: i32 = 5;
    print_number(number);
}
```

---

# If/else

```
fn my_function(number: i32) -> String {
    let result = if number > 0 {
        "Number is greater than zero"
    } else {
        "Number is not greater than zero"
    };

    result.to_owned()
}

fn main() {
    let text = my_function(5);
}
```

---

background-image: url(img/rust-memory-model.svg)
background-position: 50% 60%
background-size: 600pt

# Memory Models

---

# Ownership

```
let s1: String = String::from("hello");
let s2: String = s1;
println!("{}, world!", s1); // Compilation error: ownership is moved

let x: i32 = 5;
let y: i32 = x;
println!("x = {}, y = {}", x, y);
```

```
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:6:28

     let s1 = String::from("hello");
         -- move occurs because `s1` has type `String`
         -- which does not implement the `Copy` trait
     let s2 = s1;
              -- value moved here

     println!("{}, world!", s1);
                            ^^ value borrowed here after move
```

---

# Ownership 2

```
fn main() {
    let s1 = gives_ownership(); // moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    // s2 is moved into takes_and_gives_back,
    // which also moves its return value into s3
    let s3 = takes_and_gives_back(s2);
}
// s1 and s3 go out of scope and is dropped from the heap
// s2 goes out of scope but was moved, so nothing happens

fn gives_ownership() -> String {
    let some_string = String::from("Guten tag");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
```

---

# String vs str

```
let owned_string: String = "hello".to_owned();

let borrowed_string: &str = " world";

let mut another_owned_string = "hello ".to_owned();
another_owned_string.push_str(borrowed_string);
``` 
- `String` = growable, heap-allocated data structure

- `str` = immutable fixed-length string somewhere in memory

- _"Prefer &str as a function parameter or if you want a read-only view of a string; String when you want to own and mutate a string."_ <sup>1</sup>

.footnote[
[1] https://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html
]
---

# Make tests pass
- Code: https://github.com/arild/rust-workshop
  - 'master' is the starting point for the exercises
  - 'solution' has suggested solutions for the exercises

- Make tests in part1 and part2 pass

- Run tests with 'cargo test' in terminal, or run in IDE

- Useful resources

  - [The Rust Programming Language](https://doc.rust-lang.org/book)

  - [Rust by Example](https://doc.rust-lang.org/rust-by-example)

---

# Ownership Taxonomy

|     Type    | Ownership             | Alias? |  Mutate?  |
| :------------| :--------------------|:-----: |:---------:|
| T           | Owned                 |        | ✓         |
| &T          | Shared reference      |    ✓   |           |
| &mut T      | Mutable reference     |        | ✓         |


---


# Pattern Matching

```
enum Color {
    Blue,
    Green,
    Red
}

let color = Color::Green;

match color {
    Color::Blue => println!("Blue"),
    Color::Green => println!("Green"),
    _ => print!("Something else")
}
```

---

# Option

```
fn get_length(my_string: Option<&str>) -> Option<usize> {
    match my_string {
        Some(s) => Some(s.len()),
        None => None
    }
}

fn main() {
    let length: usize = get_length(Some("This is a string"))
        .unwrap_or(0);
}
```

---

# Structs

```
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let person = Person { name: String::from("Ola"), age: 42 };
    println!("{} is {} years old",  person.name, person.age);
}
```

---

# Vectors

```
let v1: Vec<i32> = (0..10).collect();
let v2 = vec![1, 2, 3];

let mut v3 = vec![0; 10];
v3.push(5);

//resulting vectors:
v1: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
v2: [1, 2, 3]
v3: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5]
```

---

# Iterators

```
let v1 = vec![1, 2, 3];

match v1.iter().max() {
    Some(n) => println!("max: {}", n), // n: &i32
    None => println!("None!")
}

match v1.into_iter().min() {
    Some(n) => println!("min: {}", n), // n: i32
    None => println!("None!")
}
```

---

# Closures

```
let annotated = |i: i32| -> i32 { i + 1 };
println!("{}", annotated(3));

let inferred = |i| i + 1;
my_function(inferred);

fn my_function(fun: fn(i32) -> i32) {
    println!("{}", fun(5));
}
```

---

# Map and filter

```
let v1 = vec!["cat", "dog", "elephant", "bear", "shark", "wolf"];

let v2: Vec<usize> = v1.into_iter()
    .map(|i| i.len())
    .filter(|i| *i > 3)
    .collect();

//result:
v2: [8, 4, 5, 4]

```

---

# Concurrency

- Two models of concurrency

    - Shared memory (mutexes)
    
    - Message passing (channels) 
    

- Message passing in Rust similar to Golang's model

---

# Message passing with channels

```
use crossbeam_channel::unbounded;

let (tx, rx): (Sender<String>, Receiver<String>) = unbounded();

thread::spawn(move || {
    let message = String::from("hi");

    let result = tx.send(message).unwrap(); // Send is async
});

let received = rx.recv().unwrap(); // Recv blocks until message present
println!("Got: {}", received);
```

---

background-image: url(img/tsp.png)
background-position: 15% 80%
background-size: 256pt

# Travelling Salesperson Problem (TSP)

- Definition: given a list of cities and the distances between each pair of cities, what is the shortest possible route that visits each city exactly once and returns to the origin city?

- Complexity for brute force approach: O(n!)

---


# Make tests pass
- Code: https://github.com/arild/rust-workshop

- Make tests in part3, part4, and part5 pass

- Run tests with 'cargo test' in terminal, or run in IDE

- Useful resources

  - [The Rust Programming Language](https://doc.rust-lang.org/book)

  - [Rust by Example](https://doc.rust-lang.org/rust-by-example)

---

```
/* master-slave pattern with threads and channels */

// Create work and channels for sending/receiving work and result
let work = vec![1, 2, 3, 4, 5];
let (work_send, work_recv) = unbounded();
let (result_send, result_recv) = unbounded();

// Spawn 3 threads that will carry out work until channel is dropped
for _ in 0..3 {
    let work_recv = work_recv.clone();
    let result_send = result_send.clone();

    thread::spawn(move || loop {
        match work_recv.recv() {
            Ok(number) => {
                let result = number * number; // Heavy computation
                result_send.send(result).unwrap();
            }
            Err(_) => { // Channel is dropped/disconnected
                break;
            }
        }
    });
}

for number in &work { work_send.send(*number); } // Send work on channel

for _ in 0..work.len() { // Receive all results, assuming nothing goes wrong
    let received = result_recv.recv().unwrap();
    println!("Got: {}", received);
}

drop(work_send); // The only sender is dropped, disconnecting the channel
```

---

background-image: url(img/tsp2.svg)
background-size: 300pt
# Simple TSP

---
background-image: url(img/tsp3.svg)
background-size: 400pt
background-position: 50% 65%
# Routes in TSP visualized as a tree

---
background-image: url(img/tsp4.svg)
background-size: 400pt
background-position: 50% 65%
# Branch-and-bound optimization
- Terminate immediately when shorter route is known
