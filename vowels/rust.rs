use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter any character : ");
    io::stdin().read_line(&mut input).unwrap();
    let ch = input.trim().chars().next().unwrap();
    
    input.clear();
    let vowels = "AEIOUaeiou";
    
    if vowels.contains(ch) {
        println!("Vowel");
    }
    else {
        println!("Consonant");
    }
}
