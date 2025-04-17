use std::vec;

fn main() {
    println!("Hello, world!");

    // using variable
    let var: i32 = 13;
    // above variable is immutable

    let var1: i32 = 14;
    println!("This is the value of var1 : {}", var1);

    let var1: i32 = 17;
    println!("This is the new value of var1 : {}", var1);

    let name1: &str = "Name1";
    println!("This is my name : {:?}", name1);


    let arr: [i32; 3] = [13, 21, 13];
    println!("The second element is {}", arr[1]);

    let mut dy_arr = vec![12, 43, 52, 133];
    dy_arr.push(789);
    println!("The first  element is {}", dy_arr[0]);

    let res: Option<i32> = dy_arr.pop();
    println!("The popped element is {:?}", res);


}
