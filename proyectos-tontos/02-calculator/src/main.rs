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

    calc01.add(5);
    calc01.add(15);
    calc01.subtract(3);
    println!("Calculator 01 Total: {}", calc01.get_total().to_string());

    calc01.multiply(4);
    println!("Calculator 01 Total: {}", calc01.get_total().to_string());

    let result = calc01.divide(2);
    match result {
        Ok(_) => println!("Calculator 01 Total: {}", calc01.get_total().to_string()),
        Err(e) => println!("Division failed: {}", e),
    }

    let result = calc01.divide(0);
    match result {
        Ok(_) => println!("Calculator 01 Total: {}", calc01.get_total().to_string()),
        Err(e) => println!("Division failed: {}", e),
    }
        
    calc01.clear();
    println!("Calculator 01 Total: {}", calc01.get_total().to_string());    
}