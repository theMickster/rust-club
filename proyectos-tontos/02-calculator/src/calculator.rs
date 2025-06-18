pub struct Calculator {
    brand: String,
    model: String,
    total: i32,
}

impl Calculator {
    pub fn new(brand: &str, model: &str) -> Self {
        Self {
            brand: brand.to_string(),
            model: model.to_string(),
            total: 0,
        }
    }

    pub fn display(&self) -> String {
        format!("Your calculator is -> {} {}", self.brand, self.model)
    }

    pub fn clear(&mut self) {
        self.total = 0;
    }

    pub fn get_total(&self) -> i32 {
        self.total
    }

    pub fn add(&mut self, input: i32) {
        self.total += input;
    }

    pub fn subtract(&mut self, input: i32) {
        self.total -= input;
    }

    pub fn multiply(&mut self, input: i32) {
        self.total *= input;
    }

    pub fn divide(&mut self, input: i32) -> Result<i32, &str> {
        if input == 0 {
            Err("Cannot divide by zero")
        } else {
            self.total /= input;
            Ok(self.total)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_display_succeeds() {
        let sut = Calculator::new("Texas Instruments", "TI-84 Plus");
        assert_eq!(sut.display(), "Your calculator is -> Texas Instruments TI-84 Plus");
    }

    #[test]
    fn test_calculator_clear_succeeds() {
        let mut sut = Calculator::new("Texas Instruments", "TI-83 Plus");

        sut.add(10);
        assert_eq!(sut.get_total(), 10);

        sut.clear();
        assert_eq!(sut.get_total(), 0);
    }

    #[test]
    fn test_calculator_add_succeeds() {
        let test_cases = vec![(1, 2, 3), (3, 8, 11), (5, 15, 20)];

        for (a, b, expected) in test_cases {
            let mut sut = Calculator::new("Texas Instruments", "TI-95");
            sut.add(a);
            sut.add(b);
            assert_eq!(sut.get_total(), expected);
        }
    }

    #[test]
    fn test_calculator_subtract_succeeds() {
        let test_cases = vec![(5, 2, 3), (10, 8, 2), (20, 15, 5)];

        for (a, b, expected) in test_cases {
            let mut sut = Calculator::new("Texas Instruments", "TI-74");
            sut.add(a);
            sut.subtract(b);
            assert_eq!(sut.get_total(), expected);
        }
    }

    #[test]
    fn test_calculator_multiply_succeeds() {
        let test_cases = vec![(2, 3, 6), (4, 5, 20), (6, 7, 42)];

        for (a, b, expected) in test_cases {
            let mut sut = Calculator::new("Texas Instruments", "TI-81");
            sut.add(a);
            sut.multiply(b);
            assert_eq!(sut.get_total(), expected);
        }
    }

    #[test]
    fn test_calculator_divide_succeeds() {
        let test_cases = vec![(6, 3, 2), (20, 5, 4), (42, 7, 6)];

        for (a, b, expected) in test_cases {
            let mut sut = Calculator::new("Texas Instruments", "TI-84 Plus Silver Edition");
            sut.add(a);
            let result = sut.divide(b);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
            assert_eq!(sut.get_total(), expected);
        }
    }

    #[test]
    fn test_calculator_divide_returns_correct_error() {
        let mut sut = Calculator::new("Texas Instruments", "TI-84 Plus Silver Edition");
        sut.add(10);
        let result = sut.divide(0);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot divide by zero");
    }
}
