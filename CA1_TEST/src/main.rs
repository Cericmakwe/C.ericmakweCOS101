use std::io;

struct Patient {
    name: String,
    date_of_birth: String,
    email: String,
    phone_number: String,
    num_siblings: i32,
    num_children: i32,
    diagnosis: String,
    village: String,
}

fn calculate_discount(patient: &Patient) -> f64 {
    match &patient.diagnosis[..] {
        "Alzheimer" => {
            if patient.date_of_birth > "50" && patient.num_children > 4 && patient.village == "Akpabom" {
                return 0.2;
            }
        }
        "Arrhythmia" => {
            if patient.date_of_birth == "30" && patient.num_siblings > 4 && patient.village == "Ngbauji" {
                return 0.05;
            }
        }
        "Chronic Kidney Disease" => {
            if patient.date_of_birth > "40" && patient.num_siblings > 3 && patient.num_children > 3 && patient.village == "Atabrikang" {
                return 0.15;
            }
        }
        "Diabetes" => {
            if patient.date_of_birth > "28" && patient.date_of_birth < "45" && patient.num_children >= 2 && patient.num_children <= 4 && patient.village == "Okorobilom" {
                return 0.1;
            }
        }
        "Arthritis" => {
            if patient.date_of_birth > "58" && patient.num_siblings > 5 && patient.num_children > 5 && patient.village == "Emeremen" {
                return 0.1;
            }
        }
        _ => return 0.0, // Normal charges apply for other diagnoses
    }
    return 0.0; // Normal charges apply by default
}

fn main() {
    let mut patients: Vec<Patient> = Vec::new();

    for _ in 0..100 {
        let mut patient = Patient {
            name: String::new(),
            date_of_birth: String::new(),
            email: String::new(),
            phone_number: String::new(),
            num_siblings: 0,
            num_children: 0,
            diagnosis: String::new(),
            village: String::new(),
        };

        println!("Enter patient details:");
        println!("Name: ");
        io::stdin().read_line(&mut patient.name).expect("Failed to read line");

        println!("Date of Birth: ");
        io::stdin().read_line(&mut patient.date_of_birth).expect("Failed to read line");

        println!("Email: ");
        io::stdin().read_line(&mut patient.email).expect("Failed to read line");

        println!("Phone Number: ");
        io::stdin().read_line(&mut patient.phone_number).expect("Failed to read line");

        println!("Number of Siblings: ");
        let mut num_siblings_str = String::new();
        io::stdin().read_line(&mut num_siblings_str).expect("Failed to read line");
        patient.num_siblings = num_siblings_str.trim().parse().expect("Invalid input");

        println!("Number of Children: ");
        let mut num_children_str = String::new();
        io::stdin().read_line(&mut num_children_str).expect("Failed to read line");
        patient.num_children = num_children_str.trim().parse().expect("Invalid input");

        println!("Medical Diagnosis: ");
        io::stdin().read_line(&mut patient.diagnosis).expect("Failed to read line");

        println!("Village of Residence: ");
        io::stdin().read_line(&mut patient.village).expect("Failed to read line");

        let discount = calculate_discount(&patient);
        let amount = match &patient.diagnosis[..] {
            "Alzheimer" => 1_200_000,
            "Arrhythmia" => 550_000,
            "Chronic Kidney Disease" => 1_500_000,
            "Diabetes" => 800_000,
            "Arthritis" => 450_000,
            _ => 0, // No charges apply for other diagnoses
        };
        let final_amount = amount as f64 * (1.0 - discount);

        println!("Patient Information:");
        println!("Name: {}", patient.name);
        println!("Date of Birth: {}", patient.date_of_birth);
        println!("Email: {}", patient.email);
        println!("Phone Number: {}", patient.phone_number);
        println!("Number of Siblings: {}", patient.num_siblings);
        println!("Number of Children: {}", patient.num_children);
        println!("Medical Diagnosis: {}", patient.diagnosis);
        println!("Village of Residence: {}", patient.village);
        println!("Discount: {}%", discount * 100.0);
        println!("Amount: N{}", final_amount);
        println!("-------------------------");

        patients.push(patient);
    }
}