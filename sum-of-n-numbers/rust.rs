use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter any number : ");
    io::stdin().read_line(&mut input).unwrap();
    let num: i32 = input.trim().parse().unwrap();
    
    input.clear();
    let mut sum = 0;
    for i in 1..=num {
        sum+=i;
    }
    println!("{}",sum);
}
