use std::io;

fn main() {
    println!("\nStudent Information Management Systen!");

    println!("\nChigozirim Eric Makwe.");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    println!("Your name is: {}",name);
    
    println!("\n17.");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i32 = age.trim().parse().expect("Input not an integer");
    println!("Your age is: {}", age);        
}
