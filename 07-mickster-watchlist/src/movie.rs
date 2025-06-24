//! Movie data structures and generic collections.
//!
//! This module demonstrates Phase 1 of learning Rust generics: basic generic
//! structs with single type parameters and trait bounds.

use std::fmt;

/// Represents a 90s SNL cast comedy movie.
///
/// This is a concrete (non-generic) struct that serves as the data type
/// we'll use throughout our generic demonstrations.
#[derive(Debug, Clone, PartialEq)]
pub struct Movie {
    pub title: String,
    pub director: String,
    pub year: u16,
    pub lead_actor: String,
}

impl Movie {
    /// Creates a new Movie instance.
    ///
    /// # Arguments
    ///
    /// * `title` - The movie title
    /// * `director` - The director's name
    /// * `year` - Year of release
    /// * `lead_actor` - The lead actor's name
    pub fn new(title: String, director: String, year: u16, lead_actor: String) -> Self {
        Movie {
            title,
            director,
            year,
            lead_actor,
        }
    }
}

impl fmt::Display for Movie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "🎬 {} ({}), directed by {}, starring {} 🎬",
            self.title, self.year, self.director, self.lead_actor
        )
    }
}

/// A generic collection that can hold ANY type
/// The `<T>` means "this works with any type T"
pub struct MovieCollection<T> {
    items: Vec<T>,
}

impl<T> MovieCollection<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// This implementation ONLY works when T implements Display
impl<T: fmt::Display> MovieCollection<T> {
    pub fn display_all(&self) {
        println!("\n📚 Collection ({} items):", self.len());
        for (i, item) in self.items.iter().enumerate() {
            println!("  {}. {}", i + 1, item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_movie_fixture() -> Movie {
        Movie::new(
            "Billy Madison".to_string(),
            "Tamra Davis".to_string(),
            1995,
            "Adam Sandler".to_string(),
        )
    }

    #[test]
    fn new_movie(){
        let result = get_movie_fixture();

        assert_eq!(result.title, "Billy Madison");
        assert_eq!(result.director, "Tamra Davis");
        assert_eq!(result.year, 1995);
        assert_eq!(result.lead_actor, "Adam Sandler");
    }

    #[test]
    fn display_movie(){
        let movie = get_movie_fixture();
        let result = format!("{}", movie);
        const EXPECTED: &str = "🎬 Billy Madison (1995), directed by Tamra Davis, starring Adam Sandler 🎬";
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn generic_collection_with_movies() {
        let mut collection = MovieCollection::new();
        collection.add(get_movie_fixture());
        
        assert_eq!(collection.len(), 1);
        assert!(!collection.is_empty());
    }

    #[test]
    fn generic_collection_with_strings() {
        let mut collection: MovieCollection<String> = MovieCollection::new();
        collection.add("Happy Gilmore".to_string());
        collection.add("Tommy Boy".to_string());
        
        assert_eq!(collection.len(), 2);
    }

    #[test]
    fn generic_collection_with_numbers() {
        let mut collection: MovieCollection<u16> = MovieCollection::new();
        collection.add(1995);
        collection.add(1996);
        collection.add(2003);
        
        assert_eq!(collection.len(), 3);
    }

    #[test]
    fn generic_collection_display_all_works() {
        let mut collection = MovieCollection::new();
        collection.add(get_movie_fixture());
        
        // This compiles because Movie implements Display...
        collection.display_all();
    }
}