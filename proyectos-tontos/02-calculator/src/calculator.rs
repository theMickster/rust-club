
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

}