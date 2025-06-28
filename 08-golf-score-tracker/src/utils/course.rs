use std::collections::BTreeMap;
use std::collections::HashMap;

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

pub type CourseParGenerator = fn() -> BTreeMap<u8, u8>;

pub fn get_course_catalog() -> HashMap<String, CourseParGenerator> { 
    let mut catalog: HashMap<String, CourseParGenerator> = HashMap::new();
    catalog.insert("Augusta_National".to_string(), create_august_national_pars);
    catalog.insert("Pebble_Beach".to_string(), create_pebble_beach_pars);
    catalog.insert("St_Andrews".to_string(), create_st_andrews_pars);
    catalog.insert("Torrey_Pines_North".to_string(), torrey_pines_north_course_pars);
    catalog.insert("Torrey_Pines_South".to_string(), torrey_pines_south_course_pars);
    catalog
}

pub fn get_course_pars(course_name: &str, holes: u8) -> BTreeMap<u8, u8> {
    let catalog = get_course_catalog();
    
    if let Some(generator) = catalog.get(course_name) {
        generator()
    } else {
        create_standard_pars(holes)
    }
}

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