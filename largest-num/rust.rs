use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter num 1 : ");
    io::stdin().read_line(&mut input).unwrap();
    let num1: i32 = input.trim().parse().unwrap();
    
    input.clear();
    
    println!("Enter num 2 : ");
    io::stdin().read_line(&mut input).unwrap();
    let num2: i32 = input.trim().parse().unwrap();
    
    input.clear();
    
    println!("Enter num 2 : ");
    io::stdin().read_line(&mut input).unwrap();
    let num3: i32 = input.trim().parse().unwrap();
    
    input.clear();
    
    if num1 > num2 {
        if num1 > num3 {
            println!("{}",num1);
        }
    }
    else if num2 > num3 {
        println!("{}",num2);
    }
    else {
        println!("{}",num3);
    }
}
