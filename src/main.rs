use std::io;

fn calculate_present_value(cash_flows: &[f64], discount_rate: f64) -> f64 {
    let mut present_value = 0.0;
    for (t, cash_flow) in cash_flows.iter().enumerate() {
        present_value += cash_flow / (1.0 + discount_rate).powi(t as i32);
    }
    present_value
}

fn main() {
    println!("Financial Analyzer");

    // Read future cash flows from the user.
    let mut cash_flows = Vec::new();
    loop {
        println!("Enter the cash flow (or 'q' to finish): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim().to_lowercase() == "q" {
            break;
        }

        let cash_flow: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        cash_flows.push(cash_flow);
    }

    // Read the discount rate from the user.
    let mut discount_rate_input = String::new();
    println!("Enter the discount rate (e.g., 0.1 for 10%): ");
    io::stdin()
        .read_line(&mut discount_rate_input)
        .expect("Failed to read line");

    let discount_rate: f64 = match discount_rate_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let present_value = calculate_present_value(&cash_flows, discount_rate);

    println!("Present value of cash flows: ${:.2}", present_value);
}
