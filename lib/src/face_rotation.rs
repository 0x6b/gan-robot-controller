use std::{fmt::Display, str::FromStr};

use rand::seq::SliceRandom;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FaceRotation {
    R,
    R2,
    R2Prime,
    RPrime,
    F,
    F2,
    F2Prime,
    FPrime,
    D,
    D2,
    D2Prime,
    DPrime,
    L,
    L2,
    L2Prime,
    LPrime,
    B,
    B2,
    B2Prime,
    BPrime,
    Invalid,
}

impl From<FaceRotation> for u8 {
    fn from(r: FaceRotation) -> u8 {
        use FaceRotation::*;
        match r {
            R => 0,
            R2 => 1,
            R2Prime => 1,
            RPrime => 2,
            F => 3,
            F2 => 4,
            F2Prime => 4,
            FPrime => 5,
            D => 6,
            D2 => 7,
            D2Prime => 7,
            DPrime => 8,
            L => 9,
            L2 => 10,
            L2Prime => 10,
            LPrime => 11,
            B => 12,
            B2 => 13,
            B2Prime => 13,
            BPrime => 14,
            Invalid => 255,
        }
    }
}

impl From<&FaceRotation> for u8 {
    fn from(r: &FaceRotation) -> u8 {
        u8::from(*r)
    }
}

impl From<String> for FaceRotation {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "r" => FaceRotation::R,
            "r2" => FaceRotation::R2,
            "r2'" => FaceRotation::R2Prime,
            "r'" => FaceRotation::RPrime,
            "f" => FaceRotation::F,
            "f2" => FaceRotation::F2,
            "f2'" => FaceRotation::F2Prime,
            "f'" => FaceRotation::FPrime,
            "d" => FaceRotation::D,
            "d2" => FaceRotation::D2,
            "d2'" => FaceRotation::D2Prime,
            "d'" => FaceRotation::DPrime,
            "l" => FaceRotation::L,
            "l2" => FaceRotation::L2,
            "l2'" => FaceRotation::L2Prime,
            "l'" => FaceRotation::LPrime,
            "b" => FaceRotation::B,
            "b2" => FaceRotation::B2,
            "b2'" => FaceRotation::B2Prime,
            "b'" => FaceRotation::BPrime,
            _ => FaceRotation::Invalid,
        }
    }
}

impl From<&str> for FaceRotation {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

impl FromStr for FaceRotation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FaceRotation::*;
        match s {
            "R" => Ok(R),
            "R2" => Ok(R2),
            "R2'" => Ok(R2Prime),
            "R'" => Ok(RPrime),
            "F" => Ok(F),
            "F2" => Ok(F2),
            "F2'" => Ok(F2Prime),
            "F'" => Ok(FPrime),
            "D" => Ok(D),
            "D2" => Ok(D2),
            "D2'" => Ok(D2Prime),
            "D'" => Ok(DPrime),
            "L" => Ok(L),
            "L2" => Ok(L2),
            "L2'" => Ok(L2Prime),
            "L'" => Ok(LPrime),
            "B" => Ok(B),
            "B2" => Ok(B2),
            "B2'" => Ok(B2Prime),
            "B'" => Ok(BPrime),
            _ => Ok(Invalid),
        }
    }
}

impl Display for FaceRotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FaceRotation::*;
        let s = match self {
            R => "R",
            R2 => "R2",
            R2Prime => "R2'",
            RPrime => "R'",
            F => "F",
            F2 => "F2",
            F2Prime => "F2'",
            FPrime => "F'",
            D => "D",
            D2 => "D2",
            D2Prime => "D2'",
            DPrime => "D'",
            L => "L",
            L2 => "L2",
            L2Prime => "L2'",
            LPrime => "L'",
            B => "B",
            B2 => "B2",
            B2Prime => "B2'",
            BPrime => "B'",
            Invalid => "(Invalid)",
        };
        write!(f, "{s}")
    }
}

pub struct FaceRotationMap {
    map: Vec<FaceRotation>,
}

impl Default for FaceRotationMap {
    fn default() -> Self {
        Self::new()
    }
}

impl FaceRotationMap {
    pub fn new() -> Self {
        use FaceRotation::*;
        let map = vec![
            R, R2, R2Prime, RPrime, F, F2, F2Prime, FPrime, D, D2, D2Prime, DPrime, L, L2, L2Prime,
            LPrime, B, B2, B2Prime, BPrime,
        ];
        Self { map }
    }

    pub fn get_random_moves(&self, n: usize) -> Vec<FaceRotation> {
        self.map
            .choose_multiple(&mut rand::thread_rng(), n)
            .cloned()
            .collect()
    }
}
