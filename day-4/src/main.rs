#![allow(unreachable_code, unused_labels)]

fn main() {
    println!("Hello, world! to flow-control");

    let n = 3;
    // if - else
    if n > 3 {
        println!("Number is bigger than 3!");
    } else if n == 0{
        println!("Number is Zero it is {:?}!", n);
    } else {
        println!("Number is greater than 3");
    }

    if n <= 3 && n < 5 {
        println!("Number is in-between 3 and 5");
    }


    // infinite-loops
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("Count has reached 3");
        } else if count == 7 {
            println!("Count has reached 7");
        }

        if count == 10 {
            println!("It has reached it's limit of {:}", count);
            break;
        }
    };

    
    // nested loop
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");


    // returning-out of loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter *2;
        }
    };
    println!("Result: {:}", result);


    // fizz-buzz
    const MAX_COUNT: i32 = 20;
    let mut count = 0;
    while count <= MAX_COUNT {
        if count % 3 == 0 && count % 5 == 0 {
            println!("FizzBuzz: {:}", count);
        }
        else if count % 3 == 0 {
            println!("Fizz: {:}", count);
        }
        else if count % 5 == 0 {
            println!("Bizz: {:}", count);
        }
        count += 1
    }


    // for-range, FizzBuzz with for
    for count in 1..20 { //1 is considered, 20 is exclusive
        if count % 3 == 0 && count % 5 == 0 {
            println!("FizzBuzz: {:}", count);
        }
        else if count % 3 == 0 {
            println!("Fizz: {:}", count);
        }
        else if count % 5 == 0 {
            println!("Bizz: {:}", count);
        }
    }

    for count in 1..=20 {       // 1 is inclusive, 20 is also inclusive
        println!("Count is {:}", count);
    }


    // iter vs iter_into
    let v = vec![1, 2, 3, 4, 5];

    for each in v.iter(){           // each is &i32, referenced, borrows
        println!("Each is {}", each);
    }
    println!("Vector is {:?}", v);
    
    for each in v.into_iter() {     // into_iter has i32, takes ownership
        println!("Each si {}", each);
    }
    // println!("Vector is {:?}", v);
    // v cannot be used here

    // Ownership vs borrowing
    // Concept	Toy Analogy	Rust Example
    // Ownership	Only you own the toy	let toy = String::from("Car");
    // Move Ownership	Give toy away, can't use it anymore	let friend = toy;
    // Immutable Borrow	Friends can look but not change	let friend = &toy;
    // Mutable Borrow	One friend can change, no one else uses it	let friend = &mut toy;


    let mut v = vec![1, 2, 3, 4, 5];    // iter_mut
    for each in v.iter_mut() {
        println!("each is {}", each);
    }
    println!("v is {:?}", v);

    // match (same like switch)
    let num = 13;
    match num{
        1 => println!("One"),
        3 | 5 | 7 => println!("Either 3, 5, 7"),
        10..=19 => print!("Teen"),
        _ => println!("Default")

    }

}
