use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Move {
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
}

pub struct MoveMap {
    map: HashMap<Move, u8>,
}

impl Default for MoveMap {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveMap {
    pub fn new() -> Self {
        use Move::*;
        let mut map = HashMap::new();
        map.insert(R, 0);
        map.insert(R2, 1);
        map.insert(R2Prime, 1);
        map.insert(RPrime, 2);
        map.insert(F, 3);
        map.insert(F2, 4);
        map.insert(F2Prime, 4);
        map.insert(FPrime, 5);
        map.insert(D, 6);
        map.insert(D2, 7);
        map.insert(D2Prime, 7);
        map.insert(DPrime, 8);
        map.insert(L, 9);
        map.insert(L2, 10);
        map.insert(L2Prime, 10);
        map.insert(LPrime, 11);
        map.insert(B, 12);
        map.insert(B2, 13);
        map.insert(B2Prime, 13);
        map.insert(BPrime, 14);
        Self { map }
    }

    pub fn get(&self, mv: Move) -> u8 {
        *self.map.get(&mv).unwrap()
    }

    pub fn get_random_moves(&self, n: usize) -> Vec<u8> {
        use rand::{seq::SliceRandom, thread_rng};
        let moves = self.map.keys().cloned().collect::<Vec<_>>();
        moves
            .choose_multiple(&mut thread_rng(), n)
            .map(|m| self.get(*m))
            .collect()
    }
}
