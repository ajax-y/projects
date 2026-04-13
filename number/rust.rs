use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number : ");
    io::stdin().read_line(&mut input).unwrap();
    let num: i32 = input.trim().parse().unwrap();
    
    input.clear();
    
    if num>0 {
        println!("Positive");
    }
    else if num<0 {
        println!("Negative");
    }
    else {
        println!("Zero");
    }
}
