use std::io;

fn main () {
    println!("Enter a number : ");
    let mut num = String::new();
     io::stdin()
        .read_line(&mut num)
        .expect ("Failed to read");
    
    println!("{}",num);
}
