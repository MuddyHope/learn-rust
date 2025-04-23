# Borrowing and References


Instead of taking ownership in the main.rs file, we have main function that is borrowed by calculate_fn.

There are 2 types of borrowing <br>
* Immutable -> &T
* Mutable   -> &mut T


There is concept called as scoped borrowing, where both mutable and immmutable borrowing can be

```rust 
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &s;
        println!("r1: {}", r1);
    } // r1 goes out of scope here

    let r2 = &mut s;
    r2.push_str(", world!");
    println!("r2: {}", r2);
}
```
The above example shows scoped borrowing, r1 -> Scope 1 borrows immutable 
whereas r2 borrows mutable

