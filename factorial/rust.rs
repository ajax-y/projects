use std::io::{self, Write};

fn fact(num:u128)->u128 {
    if num == 0 || num == 1 {
        return 1;
    }
    return num*fact(num-1);
}

fn main() {
    let mut input = String::new();
    print!("Enter any number : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let n: u128 = input.trim().parse().unwrap();
    
    input.clear();
    
    print!("{}",fact(n));
}
