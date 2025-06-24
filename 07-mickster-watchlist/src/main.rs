use mickster_movie_watchlist::{
    Movie, MovieCollection, MovieQueue, Rating, RatingScale, Rated, Watchlist,
};

fn main() {
    println!("🎬 THE MICKSTER'S 90s SNL COMEDY WATCHLIST 🎬\n");
    
    let collection = phase_1_generic_movie_collection();
        
    collection.display_all();
        
    phase_2_multiple_generics_with_ratings();
        
    phase_3_filters_generic_functions_and_closures();
        
    phase_4_custom_iterator_with_movie_queue();

    phase_5_whole_enchilada();
    
    println!("\n✨ THE END - You've mastered Rust generics! ✨");
}

/// Phase 1: Basic Generic Collections
/// 
/// **Demonstrates:**
/// - Generic struct with single type parameter `<T>`
/// - Basic trait bound `T: Display` for conditional methods
/// - How one struct can work with ANY type (Movie, String, numbers, etc.)
/// 
/// **Key Concepts:**
/// - `MovieCollection<T>` - The `<T>` means "works with any type T"
/// - `impl<T: Display>` - Method ONLY available when T implements Display
/// - Type safety at compile time - can't call `display_all()` on non-Display types
/// 
/// **Real-world use:** Libraries that need to work with multiple types (Vec, HashMap, Option, Result)
fn phase_1_generic_movie_collection() -> MovieCollection<Movie> {
    println!("=== Phase 1: Generic Collections ===");
    let mut collection: MovieCollection<Movie> = MovieCollection::new();
    
    collection.add(Movie::new(
        "Happy Gilmore".to_string(),
        "Dennis Dugan".to_string(),
        1996,
        "Adam Sandler".to_string(),
    ));
    
    collection.add(Movie::new(
        "Tommy Boy".to_string(),
        "Peter Segal".to_string(),
        1995,
        "Chris Farley".to_string(),
    ));
    collection
}

/// Phase 2: Multiple Generic Parameters
/// 
/// **Demonstrates:**
/// - Structs with TWO generic parameters `<T, R>`
/// - Trait bounds on struct definitions (`where T: Copy + PartialOrd`)
/// - Generic enums for type-safe configuration (RatingScale)
/// 
/// **Key Concepts:**
/// - `Rating<T>` - Generic rating where T can be u8, f32, i32, etc.
/// - `Rated<T, R>` - Pairs ANY item (T) with ANY rating type (R)
/// - Trait bounds prevent invalid types (can't use Rating<String>)
/// 
/// **Real-world use:** Result<T, E>, HashMap<K, V>, generic data structures with multiple type parameters
fn phase_2_multiple_generics_with_ratings() {
    println!("\n=== Phase 2: Rated Movies (Multiple Generics) ===");
    
    let billy_madison = Movie::new(
        "Billy Madison".to_string(),
        "Tamra Davis".to_string(),
        1995,
        "Adam Sandler".to_string(),
    );
    let rating = Rating::new(5, RatingScale::Stars);
    let rated_billy = Rated::new(billy_madison, rating);
    println!("{}", rated_billy);
    
    let elf = Movie::new(
        "Elf".to_string(),
        "Jon Favreau".to_string(),
        2003,
        "Will Ferrell".to_string(),
    );
    let elf_rating = Rating::new(9, RatingScale::Numeric);
    let rated_elf = Rated::new(elf, elf_rating);
    println!("{}", rated_elf);
}

/// Phase 3: Generic Functions with Closures
/// 
/// **Demonstrates:**
/// - Generic closures as function parameters `<F: Fn(&T) -> bool>`
/// - Type-specific implementations (`impl<U, R> Watchlist<Rated<U, R>>`)
/// - Where clauses for complex trait bounds
/// - Multiple impl blocks with different constraints
/// 
/// **Key Concepts:**
/// - `filter_by<F>` - F is a GENERIC CLOSURE that works with any T
/// - `find_by<F>` - Passes closure to iterator methods
/// - Type-specific methods - `rated_above()` ONLY exists for `Watchlist<Rated<U, R>>`
/// - Compiler ensures closures match the required signature
/// 
/// **Real-world use:** Iterator methods (map, filter, find), sorting with custom comparators, callbacks
fn phase_3_filters_generic_functions_and_closures() {
    println!("\n=== Phase 3: Watchlist with Filtering (Closures) ===");
    let mut watchlist = Watchlist::new("Farley Films".to_string());
    
    watchlist.add(Movie::new(
        "Tommy Boy".to_string(),
        "Peter Segal".to_string(),
        1995,
        "Chris Farley".to_string(),
    ));
    
    watchlist.add(Movie::new(
        "Black Sheep".to_string(),
        "Penelope Spheeris".to_string(),
        1996,
        "Chris Farley".to_string(),
    ));
    
    watchlist.add(Movie::new(
        "Beverly Hills Ninja".to_string(),
        "Dennis Dugan".to_string(),
        1997,
        "Chris Farley".to_string(),
    ));
    
    watchlist.add(Movie::new(
        "Joe Dirt".to_string(),
        "Dennie Gordon".to_string(),
        2001,
        "David Spade".to_string(),
    ));
    
    watchlist.display_all();
    
    // Filter by actor using closures!
    println!("\nFiltering for Chris Farley movies:");
    let farley_movies = watchlist.filter_by(|m| m.lead_actor == "Chris Farley");
    println!("Found {} Chris Farley films!", farley_movies.len());
    for movie in farley_movies {
        println!("  - {}", movie.title);
    }
}

/// Phase 4: Custom Iterators with Lifetimes
/// 
/// **Demonstrates:**
/// - Custom iterator implementation with Iterator trait
/// - Lifetime parameters (`'a`) that tie references to original data
/// - IntoIterator trait for ergonomic `for item in &collection` syntax
/// - Associated types (`type Item = &'a T`)
/// - Borrowing vs consuming iterators
/// 
/// **Key Concepts:**
/// - `MovieQueueIter<'a, T>` - The `'a` lifetime ensures references are valid
/// - `impl Iterator` - Requires `type Item` and `fn next()`
/// - `impl IntoIterator for &'a MovieQueue<T>` - Enables `for item in &queue`
/// - Queue remains usable after iteration (borrowing, not consuming)
/// 
/// **Real-world use:** Every Rust collection (Vec, HashMap, etc.), custom data structures that need iteration
/// 
/// **The Most Rusty Pattern:** This is THE fundamental pattern in Rust!
fn phase_4_custom_iterator_with_movie_queue() {
    println!("\n=== Phase 4: Watch Queue (Custom Iterator) ===");
    let mut queue = MovieQueue::new();
    
    queue.enqueue(Movie::new(
        "Wayne's World".to_string(),
        "Penelope Spheeris".to_string(),
        1992,
        "Mike Myers".to_string(),
    ));
    
    queue.enqueue(Movie::new(
        "Anchorman".to_string(),
        "Adam McKay".to_string(),
        2004,
        "Will Ferrell".to_string(),
    ));
    
    queue.enqueue(Movie::new(
        "Old School".to_string(),
        "Todd Phillips".to_string(),
        2003,
        "Will Ferrell".to_string(),
    ));
    
    println!("Up next to watch ({} movies):", queue.len());
    // Using IntoIterator - the ergonomic way!
    for (i, movie) in (&queue).into_iter().enumerate() {
        println!("  {}. {} ({})", i + 1, movie.title, movie.year);
    }
    
    println!("\nDequeueing and watching...");
    if let Some(movie) = queue.dequeue() {
        println!("🍿 Now watching: {}", movie);
    }
    
    println!("\nRemaining in queue: {}", queue.len());
}

/// BONUS: Combining Everything Together
/// 
/// **Demonstrates:**
/// - Using multiple generic concepts in one workflow
/// - How generics compose: `Watchlist<Rated<Movie, Rating<u8>>>`
/// - Type-specific methods that only exist for certain generic configurations
/// - Real-world application of all learned concepts
/// 
/// **Key Concepts:**
/// - Nested generics work seamlessly
/// - `rated_above()` and `highest_rated()` ONLY available for `Watchlist<Rated<U, R>>`
/// - Type inference handles complex generic types
/// - Generic code is just as performant as hand-written specific code (zero-cost abstraction)
/// 
/// **The Power of Generics:**
/// - Write once, use with infinite types
/// - Type safety at compile time (no runtime overhead)
/// - Code reusability without sacrificing performance
/// - Rust's killer feature for systems programming!
fn phase_5_whole_enchilada() {
    println!("\n=== BONUS: Rated Watchlist (Everything Together!) ===");
    let mut rated_watchlist = Watchlist::new("Top Rated 90s Comedies".to_string());
    
    let movie1 = Movie::new(
        "Happy Gilmore".to_string(),
        "Dennis Dugan".to_string(),
        1996,
        "Adam Sandler".to_string(),
    );
    rated_watchlist.add(Rated::new(movie1, Rating::new(5, RatingScale::Stars)));
    
    let movie2 = Movie::new(
        "Tommy Boy".to_string(),
        "Peter Segal".to_string(),
        1995,
        "Chris Farley".to_string(),
    );
    rated_watchlist.add(Rated::new(movie2, Rating::new(5, RatingScale::Stars)));
    
    let movie3 = Movie::new(
        "Joe Dirt".to_string(),
        "Dennie Gordon".to_string(),
        2001,
        "David Spade".to_string(),
    );
    rated_watchlist.add(Rated::new(movie3, Rating::new(3, RatingScale::Stars)));
    
    println!("\nAll rated movies:");
    rated_watchlist.display_all();
    
    // Type-specific method only available for Watchlist<Rated<U, R>>!
    println!("\nMovies rated 4 stars or higher:");
    let high_rated = rated_watchlist.rated_above(4);
    for rated in high_rated {
        println!("  ⭐ {}", rated.item.title);
    }
    
    if let Some(best) = rated_watchlist.highest_rated() {
        println!("\n🏆 Highest rated: {} - {}", best.item.title, best.rating);
    }
    }


