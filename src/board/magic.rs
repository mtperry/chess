
use crate::board::*;

pub struct Magic {
    pub mask:   BB,
    pub magic:  u64,
    pub shift:  u32,
    pub offset: usize,
}

pub fn generate_magics() -> Vec<Magic> {
    let magics:  Vec<Magic>  = Vec::new();
    let attacks: [BB; 4096]  = [BB(0); 4096];
    let version: [u32; 4096] = [0;     4096];

    let magic_mult: u64 = rand::random();

    
    for sq in ALL_SQUARES.iter() {
    
        let relevant_bits = mask.pop_count();
    }
    magics
}


