// Learning about borowwing and references


fn main() {
    println!("Hello, world!");

    let s1 = String::from("Hello");
    
    let l1n = calculate_length(&s1);

    println!("The length for s1: '{s1}' is {l1n}");

    mutable_borrow();
}




fn calculate_length(s: &String) -> usize {
    s.len()
}


// In the above example, the reference is borrowed from main function to the calculate_length function


fn mutable_borrow() {
    let mut s = String::from("hello");

    let r = &mut s;
    r.push_str(", world");

    println!("{}", r);
}



