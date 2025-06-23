use std::fmt;
use crate::rating::{ Rated};

#[derive(Debug, Clone)]
pub struct Watchlist<T> {
    items: Vec<T>,
    name: String
}

impl<T> Watchlist<T> {
    pub fn new(name: String) -> Self {
        Self {
            items: Vec::new(),
            name
        }
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

    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Find the first item that matches a predicate
    /// The predicate is a GENERIC CLOSURE!
    /// F = any function that takes &T and returns bool
    pub fn find_by<F>(&self, predicate: F) -> Option<&T>
        where F: Fn(&T) -> bool,
    {
        self.items.iter().find(|item| predicate(item))
    }

    /// Filter items by a predicate - returns a new Vec!
    pub fn filter_by<F>(&self, predicate: F) -> Vec<&T>
        where F: Fn(&T) -> bool,
    {
        self.items.iter().filter(|item| predicate(item)).collect()
    }
}

impl<T> Watchlist<T> where T: Clone,
{
    /// Get all items as a cloned Vec and is only available when T is Clone
    pub fn get_all(&self) -> Vec<T> {
        self.items.clone()
    }
}

impl<T> Watchlist<T> where T: Clone + PartialOrd,
{
    /// Sort items and return a new sorted Vec. Requires BOTH Clone AND PartialOrd
    pub fn sorted(&self) -> Vec<T> {
        let mut sorted = self.items.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted
    }

    /// Get the top N items
    pub fn top_n(&self, n: usize) -> Vec<T> {
        let mut sorted = self.sorted();
        sorted.reverse();
        sorted.into_iter().take(n).collect()
    }
}

impl<U, R> Watchlist<Rated<U, R>>
    where U: fmt::Display + Clone,
          R: Copy + PartialOrd + fmt::Display,
{
    /// Get items with rating higher than threshold
    pub fn rated_above(&self, threshold: R) -> Vec<&Rated<U, R>> {
        self.items
            .iter()
            .filter(|rated| rated.rating.value() > threshold)
            .collect()
    }

    /// Get the highest rated item
    pub fn highest_rated(&self) -> Option<&Rated<U, R>> {
        self.items
            .iter()
            .max_by(|a, b| a.rating.value().partial_cmp(&b.rating.value()).unwrap())
    }
}

impl<T: fmt::Display> Watchlist<T> {
    pub fn display_all(&self) {
        println!("\n🎬 {} ({} items):", self.name, self.len());
        for (i, item) in self.items.iter().enumerate() {
            println!("  {}. {}", i + 1, item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rating;
    use crate::movie::Movie;
    use crate::rating::RatingScale;

    fn get_movie_fixture_01() -> Movie {
        Movie::new(
            "Billy Madison".to_string(),
            "Tamra Davis".to_string(),
            1995,
            "Adam Sandler".to_string(),
        )
    }

    fn get_movie_fixture_02() -> Movie {
        Movie::new(
            "Tommy Boy".to_string(),
            "Peter Segal".to_string(),
            1995,
            "Chris Farley".to_string(),
        )
    }

    fn get_movie_fixture_03() -> Movie {
        Movie::new(
            "Black Sheep".to_string(),
            "Penelope Spheeris".to_string(),
            1996,
            "Chris Farley".to_string(),
        )
    }

    fn get_movie_fixture_04() -> Movie {
        Movie::new(
            "Joe Dirt".to_string(),
            "Dennie Gordon".to_string(),
            2001,
            "David Spade".to_string(),
        )
    }

    #[test]
    fn new_watchlist() {
        let result: Watchlist<Movie> = Watchlist::new("Mick's List".to_string());

        assert_eq!(result.len(), 0);
        assert_eq!(result.name(), "Mick's List");
    }

    #[test]
    fn find_by_with_closure() {
        let mut watchlist = Watchlist::new("Find by with Closures".to_string());
        watchlist.add(get_movie_fixture_01());
        watchlist.add(get_movie_fixture_02());

        let result = watchlist.find_by(|m| m.title == "Tommy Boy");
        assert!(result.is_some());
        assert_eq!(result.unwrap().lead_actor, "Chris Farley");

        assert!( watchlist.find_by(|x| x.director == "MickLetofsky").is_none());
    }

    #[test]
    fn filter_by_year() {
        let mut watchlist = Watchlist::new("Filter by Year".to_string());
        watchlist.add(get_movie_fixture_01());
        watchlist.add(get_movie_fixture_02());
        watchlist.add(get_movie_fixture_03());

        let result = watchlist.filter_by(|x| x.year == 1996);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].title, "Black Sheep");
    }

    #[test]
    fn filter_by_actor() {
        let mut watchlist = Watchlist::new("Filter by Actor".to_string());
        watchlist.add(get_movie_fixture_01());
        watchlist.add(get_movie_fixture_02());
        watchlist.add(get_movie_fixture_03()); 

        let result = watchlist.filter_by(|m| m.lead_actor == "Chris Farley");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn rated_watchlist_operations() {
        let mut watchlist = Watchlist::new("Rated watchlist".to_string());
        let rating = Rating::new(5, RatingScale::Stars);
        watchlist.add( Rated::new(get_movie_fixture_01(), rating));

        let rating = Rating::new(4, RatingScale::Stars);
        watchlist.add( Rated::new(get_movie_fixture_02(), rating));

        let rating = Rating::new(3, RatingScale::Stars);
        watchlist.add( Rated::new(get_movie_fixture_04(), rating));

        let high_ratings = watchlist.rated_above(3);
        let best = watchlist.highest_rated();
        
        assert_eq!(high_ratings.len(), 2);
        assert_eq!(best.unwrap().item.title, "Billy Madison");
    }

}