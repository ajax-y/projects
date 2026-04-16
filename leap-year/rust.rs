use std::io;

fn main() {
    let mut input = String::new();
    print!("Enter any year : ");
    io::stdin().read_line(&mut input).unwrap();
    let year: i32 = input.trim().parse().unwrap();
    
    input.clear();
    
    if year%400==0 {
        print!("Leap year");
    }
    else if year%100==0 {
        print!("Not a leap year");
    }
    else if year%4==0 {
        print!("Leap year");
    }
    else {
        print!("Not a leap year");
    }
}
