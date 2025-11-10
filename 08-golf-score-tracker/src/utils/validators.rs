use crate::error::{GolfError, Result};

pub fn validate_hole_number(hole: u8, max_holes: u8) -> Result<()> {
    if hole == 0 || hole > max_holes {
        return Err(GolfError::InvalidHole { hole, max_holes });
    }
    Ok(())
}

pub fn validate_par(par: u8) -> Result<()> {
    if !(par == 3 || par == 4 || par == 5) {
        return Err(GolfError::InvalidPar(par));
    }
    Ok(())
}

pub fn validate_score(strokes: u8, hole: u8, par: u8) -> Result<()> {
    if !(1..=15).contains(&strokes) {
        return Err(GolfError::InvalidScore { score: strokes as i32, hole, par });
    }
    Ok(())
}