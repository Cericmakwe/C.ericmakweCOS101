fn main() {
    
    let fullname = "Chigozirim Eric Makwe";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";

    let mut school = "School of Science".to_string();
    // push string
    school.push_str( "and Technology");

    println!("My name is: {}", fullname);

    println!("The length my fullname is: {}",fullname.len());
    println!("I am a student of {} department", department);
    println!("{}",school);
    println!("{}",uni);
}
