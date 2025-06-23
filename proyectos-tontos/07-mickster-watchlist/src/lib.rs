//! The Mickster's Movie Watchlist
//! 
//! A fun library for managing 90s SNL movie collections using Rust generics.

mod movie;
mod rating;
mod watchlist;
mod queue;

pub use movie::{Movie, MovieCollection};
pub use rating::{Rating, RatingScale, Rated};
pub use watchlist::Watchlist;
pub use queue::MovieQueue;