use std::io;
 fn main() {
     let mut input = String::new();
     println!("Enter num 1 : ");
     io::stdin().read_line(&mut input).unwrap();
     let n1: i32 = input.trim().parse().unwrap();
     
     input.clear();
     
     println!("Enter num 2 : ");
     io::stdin().read_line(&mut input).unwrap();
     let n2: i32 = input.trim().parse().unwrap();
     
     println!("{}",n1+n2);
 }
