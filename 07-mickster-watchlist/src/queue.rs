use std::collections::VecDeque;

/// A FIFO (First In, First Out) queue for any type
/// Perfect for "watch next" lists!
#[derive(Debug, Clone)]
pub struct MovieQueue<T> {
    items: VecDeque<T>,
}

impl<T> MovieQueue<T> {
    /// Create a new empty queue
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    /// Add an item to the back of the queue
    pub fn enqueue(&mut self, item: T) {
        self.items.push_back(item);
    }

    /// Remove and return the front item
    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    /// Look at the front item without removing it
    pub fn peek(&self) -> Option<&T> {
        self.items.front()
    }

    /// Get the number of items
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Create a borrowing iterator. The queue can be used again after iteration.
    pub fn iter<'a>(&'a self) -> MovieQueueIter<'a, T> {
        MovieQueueIter {
            inner: self.items.iter(),
        }
    }
}

/// Custom iterator for MovieQueue
/// The 'a lifetime parameter says: "references returned by this iterator 
/// are tied to the lifetime of the queue we're borrowing from"
pub struct MovieQueueIter<'a, T> {
    inner: std::collections::vec_deque::Iter<'a, T>,
}

/// Implement Iterator trait - this is what makes it work in for loops!
impl<'a, T> Iterator for MovieQueueIter<'a, T> {
    // Associated type - we yield references!
    type Item = &'a T;  
    
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// Implement IntoIterator for &MovieQueue. This allows: for item in &queue { ... }
impl<'a, T> IntoIterator for &'a MovieQueue<T> {
    type Item = &'a T;
    type IntoIter = MovieQueueIter<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        MovieQueueIter {
            inner: self.items.iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::movie::Movie;

    fn get_movie_fixture() -> Movie {
        Movie::new(
            "Happy Gilmore".to_string(),
            "Dennis Dugan".to_string(),
            1996,
            "Adam Sandler".to_string(),
        )
    }

    #[test]
    fn new_queue_is_empty() {
        let queue: MovieQueue<Movie> = MovieQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn queue_fifo_behavior() {
        let mut queue = MovieQueue::new();
        
        // Enqueue 3 movies
        queue.enqueue(get_movie_fixture());
        
        let movie2 = Movie::new(
            "Tommy Boy".to_string(),
            "Peter Segal".to_string(),
            1995,
            "Chris Farley".to_string(),
        );
        queue.enqueue(movie2);
        
        let movie3 = Movie::new(
            "Elf".to_string(),
            "Jon Favreau".to_string(),
            2003,
            "Will Ferrell".to_string(),
        );
        queue.enqueue(movie3);
        
        assert_eq!(queue.len(), 3);
        assert!(!queue.is_empty());
        
        // Dequeue should return in FIFO order (First In, First Out)
        let first = queue.dequeue().unwrap();
        assert_eq!(first.title, "Happy Gilmore");
        
        let second = queue.dequeue().unwrap();
        assert_eq!(second.title, "Tommy Boy");
        
        assert_eq!(queue.len(), 1);
        
        let third = queue.dequeue().unwrap();
        assert_eq!(third.title, "Elf");
        
        assert!(queue.is_empty());
        assert_eq!(queue.dequeue(), None); // Empty queue returns None
    }

    #[test]
    fn peek_doesnt_remove() {
        let mut queue = MovieQueue::new();
        queue.enqueue(get_movie_fixture());
        
        // Peek multiple times - shouldn't remove item
        let peeked1 = queue.peek().unwrap();
        assert_eq!(peeked1.title, "Happy Gilmore");
        
        let peeked2 = queue.peek().unwrap();
        assert_eq!(peeked2.title, "Happy Gilmore");
        
        // Still has 1 item
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn iter_borrows_doesnt_consume() {
        let mut queue = MovieQueue::new();
        queue.enqueue(get_movie_fixture());
        
        let movie2 = Movie::new(
            "Tommy Boy".to_string(),
            "Peter Segal".to_string(),
            1995,
            "Chris Farley".to_string(),
        );
        queue.enqueue(movie2);
        
        // Iterate with iter() - borrows the queue
        let mut count = 0;
        for movie in queue.iter() {
            println!("Title: {}", movie.title);
            count += 1;
        }
        assert_eq!(count, 2);
        
        // Queue still usable! This proves we borrowed, not consumed!
        assert_eq!(queue.len(), 2);
        assert!(!queue.is_empty());
        
        // Can iterate AGAIN!
        let titles: Vec<_> = queue.iter().map(|m| &m.title).collect();
        assert_eq!(titles.len(), 2);
    }

    #[test]
    fn into_iterator_syntax_works() {
        let mut queue = MovieQueue::new();
        queue.enqueue(get_movie_fixture());
        
        let movie2 = Movie::new(
            "Elf".to_string(),
            "Jon Favreau".to_string(),
            2003,
            "Will Ferrell".to_string(),
        );
        queue.enqueue(movie2);
        
        // The ergonomic way - for item in &queue!
        let mut titles = Vec::new();
        for movie in &queue {  // No .iter() needed!
            titles.push(&movie.title);
        }
        
        assert_eq!(titles.len(), 2);
        assert_eq!(titles[0], "Happy Gilmore");
        assert_eq!(titles[1], "Elf");
        
        // Queue still usable!
        assert_eq!(queue.len(), 2);
    }

}
