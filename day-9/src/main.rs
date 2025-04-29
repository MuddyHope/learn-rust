// learn about Enum and match destructuring

enum Direction {
    North,
    South,
    East,
    West,
}

// enums with data

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}



fn move_robot(dir: Direction) {
    match dir {
        Direction::North => println!("Moving up"),
        Direction::South => println!("Moving down"),
        Direction::East  => println!("Moving right"),
        Direction::West  => println!("Moving left"),
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit command received."),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color RGB({}, {}, {})", r, g, b),
    }
}


fn find_number(nums: &[i32], target: i32) -> Option<usize> {
    for i in nums.iter() {
        println!("I is {}", i);
    }
    for (i, &n) in nums.iter().enumerate() {
        println!("{}, {}", i, &n);
        if n == target {
            return Some(i);
        }
    }
    None
}


fn main() {
    let dir = Direction::East;
    move_robot(dir);
    let msg1 = Message::Move { x: 10, y: 20 };
    process_message(msg1);

    let nums = [10, 20, 30];
    match find_number(&nums, 40) {
        Some(i) => println!("Found at index: {}", i),
        None => println!("Not found"),
    }
}
