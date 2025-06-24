use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RatingScale {
    Stars,         
    Numeric
}

/// A generic rating that can use ANY type T, BUT:
/// - T must be Copy (can duplicate easily)
/// - T must be PartialOrd (can compare values)
/// - T must be Display (can print)
/// 
/// This prevents someone from doing `Rating<String>` or `Rating<Vec<i32>>!`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rating<T> where T: Copy + PartialOrd + fmt::Display,
{
    pub value: T,
    pub scale: RatingScale,
}

impl<T> Rating<T> where T: Copy + PartialOrd + fmt::Display,
{
    pub fn new(value: T, scale: RatingScale) -> Self {
        Self { value, scale }
    }

    pub fn is_higher_than(&self, other: &Self) -> bool {
        self.value > other.value
    }

    pub fn value(&self) -> T {
        self.value
    }
}

impl<T> fmt::Display for Rating<T> where T: Copy + PartialOrd + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.scale {
            RatingScale::Stars => write!(f, "{} ⭐", self.value),
            RatingScale::Numeric => write!(f, "{}/10", self.value),
        }
    }
}

/// Pair ANY item with ANY rating type
/// T = the thing being rated (must implement Display)
/// R = the rating value type (must be Copy + PartialOrd + Display)
#[derive(Debug, Clone, PartialEq)]
pub struct Rated<T, R> where T: fmt::Display, R: Copy + PartialOrd + fmt::Display
{
    pub item: T,
    pub rating: Rating<R>,
}

impl<T, R> Rated<T, R> where T: fmt::Display, R: Copy + PartialOrd + fmt::Display
{
    pub fn new(item: T, rating: Rating<R>) -> Self {
        Self { item, rating }
    }
}

impl<T, R> fmt::Display for Rated<T, R> where T: fmt::Display, R: Copy + PartialOrd + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - Rating: {}", self.item, self.rating)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating_with_stars() {
        let result = Rating::new(4, RatingScale::Stars);
        assert_eq!(result.value(), 4);
    }

    #[test]
    fn rating_with_numeric() {
        let rating = Rating::new(7, RatingScale::Numeric);
        assert_eq!(rating.value(), 7);
    }

     #[test]
    fn rating_comparison() {
        let high = Rating::new(5, RatingScale::Stars);
        let low = Rating::new(3, RatingScale::Stars);
        assert!(high.is_higher_than(&low));
    }

    #[test]
    fn rated_with_string_and_u8() {
        let rating = Rating::new(6, RatingScale::Stars);
        let rated = Rated::new("Happy Gilmore".to_string(), rating);
        assert_eq!(rated.rating.value(), 6);
    }

    #[test]
    fn display_rating() {
        let rating = Rating::new(5, RatingScale::Stars);
        let display = format!("{}", rating);
        assert_eq!(display, "5 ⭐");
    }
}