use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    //Vector -> dynamically growing

    let mut temperatures: Vec<i32> = Vec::new();
    temperatures.push(25);
    temperatures.push(27);
    temperatures.push(24);
    println!("Temperatures are {:?}", temperatures);
    println!("First temperature: {}", temperatures[0]);
    // Hashmap -> (key,value) pair

    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Green", 10);

    println!("Scores are {:?}", scores);
    println!("Blue Score: {:?}", scores.get("Bleen").unwrap_or(&0));
    
}
