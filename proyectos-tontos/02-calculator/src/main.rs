mod calculator;

use calculator::Calculator;

fn main() {
    let mut calc: Calculator = Calculator::new("Texas Instruments", "TI-83 Plus");

    println!("{}", calc.display());
    phase_one(&mut calc);

    phase_two(&mut calc);

    phase_three(&mut calc);

    phase_four(&mut calc);

    phase_five(&mut calc);

    phase_six(&mut calc);
}

fn phase_one(calculator: &mut Calculator) {
    println!("");
    println!("Phase One::");
    println!("Calculator Total: {}", calculator.get_total().to_string());

    calculator.add(5);
    println!("Calculator Total: {}", calculator.get_total().to_string());

    calculator.add(15);
    println!("Calculator Total: {}", calculator.get_total().to_string());

    calculator.clear();
}

fn phase_two(calculator: &mut Calculator) {
    println!("");
    println!("Phase Two::");
    calculator.add(5);
    calculator.add(15);
    calculator.subtract(3);
    println!("Calculator Total: {}", calculator.get_total().to_string());

    calculator.multiply(4);
    println!("Calculator Total: {}", calculator.get_total().to_string());

    calculator.multiply(4);
    println!("Calculator Total: {}", calculator.get_total().to_string());
    calculator.clear();
}

fn phase_three(calculator: &mut Calculator) {
    println!("");
    println!("Phase Three::");

    let result = calculator.divide(2);
    match result {
        Ok(_) => println!("Calculator Total: {}", calculator.get_total().to_string()),
        Err(e) => println!("Division failed: {}", e),
    }

    let result = calculator.divide(0);
    match result {
        Ok(_) => println!("Calculator Total: {}", calculator.get_total().to_string()),
        Err(e) => println!("Division failed: {}", e),
    }

    println!("Calculator Total: {}", calculator.get_total().to_string());
    calculator.clear();
}

fn phase_four(calculator: &mut Calculator) {
    println!("");
    println!("Phase Four::");
    let values = [8, 30, 1, 3, 7, 19];

    for n in 0..values.len() {
        calculator.add(values[n]);
    }

    println!("Calculator Total: {}", calculator.get_total().to_string());
    calculator.clear();
}

fn phase_five(calc: &mut Calculator) {
    println!("");
    println!("Phase Five::");

    let values = [25, 7, 6, 34];

    for n in &values[0..2] {
        calc.add(*n);
    }
    println!(
        "Calculator Total after adding first two values: {}",
        calc.get_total().to_string()
    );

    for n in &values[2..4] {
        calc.subtract(*n);
    }

    println!(
        "Calculator Total after subtracting last two values: {}",
        calc.get_total().to_string()
    );
    calc.clear();
}

fn phase_six(calculator: &mut Calculator) {
    println!("");
    println!("Phase Six::");

    let mut values = vec![10, 20, 3];
    values.push(4);
    values.push(5);

    for n in values {
        calculator.multiply(n);
    }
    println!("Calculator Total: {}", calculator.get_total().to_string());
    calculator.clear();
}
