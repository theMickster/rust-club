use serde ::{Deserialize, Serialize};

use crate::error::Result;
use crate::utils::validators::{validate_hole_number, validate_par};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Hole {
    pub number: u8,
    pub par: u8
}

impl Hole {
    pub fn new(number: u8, par:u8, max_holes: u8) -> Result<Self> {
        validate_hole_number(number, max_holes)?;
        validate_par(par)?;
        Ok(Self { number, par })
    }
}