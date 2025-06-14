mod calculator;

use calculator::Calculator;

fn main() {
    let mut calc01: Calculator = Calculator::new("Texas Instruments", "TI-83 Plus");
    println!("{}", calc01.display());
    println!("Calculator 01 Total: {}", calc01.get_total().to_string());
    calc01.add(5);
    println!("Calculator 01 Total: {}", calc01.get_total().to_string());
    calc01.add(15);
    println!("Calculator 01 Total: {}", calc01.get_total().to_string());
    calc01.clear();
    println!("Calculator 01 Total: {}", calc01.get_total().to_string());    
}