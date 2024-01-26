use std::io;

fn main() {
    // Define the menu and their prices
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();

    println!("Menu: ");
    println!(" P = Poundo Yam & Edikaiko Soup = 3,200.00");
    println!(" F = Fried Rice & Chicken = 3,000.00");
    println!(" A = Amala & Ewedu Soup = 2,500.00");
    println!(" E = Eba & Egusi Soup = 2,000.00");
    println!(" W = White Rice & Stew = 2,500.00");

    // Collect quantities of each item
    println!("Amount for Poundo Yam & Edikaiko Soup");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let P: f64 = input1.trim().parse().expect("Not a valid number");

    println!("Amount for Fried Rice & Chicken");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let F: f64 = input2.trim().parse().expect("Not a valid number");

    println!("Amount for Amala & Ewedu Soup");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let A: f64 = input3.trim().parse().expect("Not a valid number");

    println!("Amount for Eba & Egusi Soup");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let E: f64 = input4.trim().parse().expect("Not a valid number");

    println!("Amount for White Rice & Stew");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let W: f64 = input5.trim().parse().expect("Not a valid number");

    // Calculate item subtotals
    let subtotal1 = P * 3200.00;
    let subtotal2 = F * 3000.00;
    let subtotal3 = A * 2500.00;
    let subtotal4 = E * 2000.00;
    let subtotal5 = W * 2500.00;

    // Calculate the order total
    let order_total = subtotal1 + subtotal2 + subtotal3 + subtotal4 + subtotal5;

    // Check if a discount applies
    if order_total > 10_000.00 {
        let discount = 0.05 * order_total;
        let discounted_total = order_total - discount;
        println!("5% Discount Applied! Discounted Total: N{:.2}", discounted_total);
    } else {
        println!("Total Order Amount: N{:.2}", order_total);
    }
}