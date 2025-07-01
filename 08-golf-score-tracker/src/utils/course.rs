//! Golf course layout definitions and utilities.
//!
//! This module provides functions to generate standard and famous golf course
//! par layouts. Course layouts are represented as `BTreeMap<u8, u8>` where
//! the key is the hole number (1-18) and the value is the par for that hole.
//!
//! # Examples
//!
//! ```
//! use golf_score_tracker::utils::{create_standard_pars, get_course_pars};
//! use std::collections::BTreeMap;
//!
//! // Create a standard 18-hole course
//! let pars = create_standard_pars(18);
//! assert_eq!(pars.len(), 18);
//!
//! // Get a famous course by name
//! let masters = get_course_pars("masters", 18);
//! assert_eq!(masters.len(), 18);
//! ```

use std::collections::BTreeMap;
use std::collections::HashMap;

/// Function pointer type for course par generation functions.
///
/// This type alias represents functions that take no arguments and return
/// a complete course layout as a `BTreeMap`.
pub type CourseParGenerator = fn() -> BTreeMap<u8, u8>;

/// Creates a standard golf course with algorithmically generated pars.
///
/// This function generates a course layout with variety based on hole numbers.
/// The pattern uses modulo arithmetic to distribute par 3s, 4s, and 5s across
/// the course.
///
/// # Arguments
///
/// * `holes` - Number of holes on the course (typically 9 or 18)
///
/// # Returns
///
/// A `BTreeMap` where keys are hole numbers (1 to `holes`) and values are
/// the par for each hole (3, 4, or 5).
///
pub fn create_standard_pars(holes: u8) -> BTreeMap<u8, u8> {
    let mut pars = BTreeMap::new();

    for hole in 1..=holes {
        let par = match hole % 3 {
            0 => 5, 
            1 => 4,
            _ => 3,
        };
        pars.insert(hole, par);
    }
    pars
}


/// Creates the Augusta National (Masters Tournament) course layout.
///
/// Returns the exact par layout used at Augusta National Golf Club, home
/// of the Masters Tournament. This is one of golf's most famous courses.
///
/// # Returns
///
/// An 18-hole course with the authentic Augusta National par values.
///
/// # Course Details
///
/// - **Total Par**: 72
/// - **Par 3s**: Holes 4, 6, 12, 16
/// - **Par 5s**: Holes 2, 8, 13, 15
/// - **Par 4s**: All remaining holes
pub fn create_august_national_pars() -> BTreeMap<u8, u8> {
    let pars = [
        (1, 4), (2, 5), (3, 4), (4, 3), (5, 4), (6, 3),
        (7, 4), (8, 5), (9, 4), (10, 4), (11, 4), (12, 3),
        (13, 5), (14, 4), (15, 5), (16, 3), (17, 4), (18, 4),
    ];
    pars.iter().copied().collect()
}

pub fn create_pebble_beach_pars() -> BTreeMap<u8, u8> {
    let pars = [
        (1, 4), (2, 5), (3, 4), (4, 3), (5, 4), (6, 5),
        (7, 3), (8, 4), (9, 4), (10, 4), (11, 4), (12, 3),
        (13, 4), (14, 5), (15, 4), (16, 3), (17, 4), (18, 5),
    ];
    pars.iter().copied().collect()
}

pub fn create_st_andrews_pars() -> BTreeMap<u8, u8> {
    let pars = [
        (1, 4), (2, 4), (3, 4), (4, 5), (5, 3), (6, 4),
        (7, 4), (8, 3), (9, 5), (10, 4), (11, 4), (12, 3),
        (13, 4), (14, 4), (15, 5), (16, 3), (17, 4), (18, 4),
    ];
    pars.iter().copied().collect()
}

pub fn torrey_pines_north_course_pars() -> BTreeMap<u8, u8> {
    let pars = [
        (1, 4), (2, 4), (3, 3), (4, 4), (5, 5), (6, 4),
        (7, 4), (8, 3), (9, 5), (10, 5), (11, 4), (12, 3),
        (13, 4), (14, 4), (15, 3), (16, 4), (17, 5), (18, 4),
    ];
    pars.iter().copied().collect()
}

pub fn torrey_pines_south_course_pars() -> BTreeMap<u8, u8> {
    let pars = [
        (1, 4), (2, 4), (3, 3), (4, 4), (5, 5), (6, 5),
        (7, 4), (8, 3), (9, 5), (10, 4), (11, 3), (12, 4),
        (13, 5), (14, 4), (15, 4), (16, 3), (17, 4), (18, 5),
    ];
    pars.iter().copied().collect()
}

/// Returns a catalog of all available named courses.
///
/// This function provides a lookup table mapping course names to their
/// par generation functions. Use this to dynamically select courses by name.
///
/// # Returns
///
/// A `HashMap` where keys are course names (lowercase, hyphenated) and
/// values are function pointers that generate the course layout.
/// 
pub fn get_course_catalog() -> HashMap<String, CourseParGenerator> { 
    let mut catalog: HashMap<String, CourseParGenerator> = HashMap::new();
    catalog.insert("Augusta_National".to_string(), create_august_national_pars);
    catalog.insert("Pebble_Beach".to_string(), create_pebble_beach_pars);
    catalog.insert("St_Andrews".to_string(), create_st_andrews_pars);
    catalog.insert("Torrey_Pines_North".to_string(), torrey_pines_north_course_pars);
    catalog.insert("Torrey_Pines_South".to_string(), torrey_pines_south_course_pars);
    catalog
}

/// Gets course pars by name, falling back to standard layout.
///
/// This is the primary function for retrieving course layouts. It checks
/// the course catalog for a named course, and if not found, generates a
/// standard layout with the specified number of holes.
///
/// # Arguments
///
/// * `course_name` - Name of the course (e.g., "masters", "pebble-beach")
/// * `holes` - Number of holes if using standard layout
///
/// # Returns
///
/// Course par layout as a `BTreeMap<u8, u8>`.
///
pub fn get_course_pars(course_name: &str, holes: u8) -> BTreeMap<u8, u8> {
    let catalog = get_course_catalog();
    
    if let Some(generator) = catalog.get(course_name) {
        generator()
    } else {
        create_standard_pars(holes)
    }
}

/// Lists all available course names.
///
/// Returns a vector of course names that can be used with `get_course_pars()`.
///
/// # Returns
///
/// Vector of course name strings (e.g., `["masters", "pebble-beach", ...]`)
///
pub fn list_available_courses() -> Vec<String> {
    get_course_catalog().keys().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_pars_nine_hole_correct_count() {
        let result = create_standard_pars(9);
        assert_eq!(result.len(), 9);
    }

    #[test]
    fn test_standard_pars_eighteen_hole_correct_count() {
        let result = create_standard_pars(18);
        assert_eq!(result.len(), 18);
    }

        #[test]
    fn masters_pars_has_18_holes() {
        let pars = create_august_national_pars();
        assert_eq!(pars.len(), 18);
        assert_eq!(*pars.get(&1).unwrap_or(&0), 4);
        assert_eq!(*pars.get(&12).unwrap_or(&0), 3);
    }
}