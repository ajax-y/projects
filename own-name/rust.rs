use std::io;

fn main () {
    println!("Enter Your Name : ");
    let mut name = String::new();
    io::stdin()
        .read_line (&mut name)
        .expect ("Failed to read");
    println!("{}", name.trim());
}
