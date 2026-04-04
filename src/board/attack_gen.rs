
use std::io::Write;

use crate::board::*;

pub fn write_attack_tables(file: &mut std::fs::File) -> std::io::Result<()> {
    let mut output = String::new();

    // Generate and write pawn attack tables
    output.push_str("static PAWN_ATTACKS: [[BB; NUM_COLORS]; NUM_SQUARES] = [\n");
    for sq in ALL_SQUARES.iter().copied() {
        let white_attacks = generate_pawn_attacks(sq, Color::White);
        let black_attacks = generate_pawn_attacks(sq, Color::Black);
        output.push_str(&format!(
            "\t[BB({:#018X}), BB({:#018X})]\n",
            white_attacks.0, black_attacks.0
        ));
    }
    output.push_str("];\n\n");

    // Generate and write knight attack tables
    output.push_str("static KNIGHT_ATTACKS: [BB; NUM_SQUARES] = [\n");
    for sq in ALL_SQUARES.iter().copied() {
        let attacks = generate_knight_attacks(sq);
        output.push_str(&format!(
            "\tBB({:#018X}),\n",
            attacks.0
        ));
    }
    output.push_str("];\n\n");

    // Generate and write king attack tables
    output.push_str("static KING_ATTACKS: [BB; NUM_SQUARES] = [\n");
    for sq in ALL_SQUARES.iter().copied() {
        let attacks = generate_king_attacks(sq);
        output.push_str(&format!(
            "\tBB({:#018X}),\n",
            attacks.0
        ));
    }
    output.push_str("];\n\n");

    file.write_all(output.as_bytes())
}

fn generate_pawn_attacks_table() -> [[BB; NUM_COLORS]; NUM_SQUARES] {
    let mut table = [[BB::EMPTY; NUM_COLORS]; NUM_SQUARES];
    for sq in ALL_SQUARES.iter().copied() {
        table[sq.to_index()][Color::White.to_index()] = generate_pawn_attacks(sq, Color::White);
        table[sq.to_index()][Color::Black.to_index()] = generate_pawn_attacks(sq, Color::Black);
    }

    table
}

fn generate_pawn_attacks(sq: SQ, color: Color) -> BB {
    let mut bb = BB::EMPTY;
    match color {
        Color::White =>{
            if let Some(new_sq) = sq.shift(Direction::NE.file_offset(), Direction::NE.rank_offset()) {
                bb = bb.with_set(new_sq);
            }
            if let Some(new_sq) = sq.shift(Direction::NW.file_offset(), Direction::NW.rank_offset()) {
                bb = bb.with_set(new_sq);
            }
        }
        Color::Black => {
            if let Some(new_sq) = sq.shift(Direction::SE.file_offset(), Direction::SE.rank_offset()) {
                bb = bb.with_set(new_sq);
            }
            if let Some(new_sq) = sq.shift(Direction::SW.file_offset(), Direction::SW.rank_offset()) {
                bb = bb.with_set(new_sq);
            }
        }
    }
    bb
}

fn generate_knight_attacks(sq: SQ) -> BB {
    let mut knight_bb = BB::EMPTY;
    let rank = sq.rank();
    let file = sq.file();
    let knight_moves = [
        (2, 1), (1, 2), (-1, 2), (-2, 1),
        (-2, -1), (-1, -2), (1, -2), (2, -1)
    ];

    for (dr, df) in knight_moves.iter() {
        let new_rank_index = rank.to_index() as isize + dr;
        let new_file_index = file.to_index() as isize + df;

        if new_rank_index >= 0 && new_rank_index < NUM_RANKS as isize &&
            new_file_index >= 0 && new_file_index < NUM_FILES as isize {
            let new_sq = SQ::from_coords(
                File::from_index(new_file_index as usize),
                Rank::from_index(new_rank_index as usize)
            );
            knight_bb = knight_bb.with_set(new_sq);
        }
    }

    knight_bb
}

fn generate_bishop_attacks(sq: SQ, occupied: BB) -> BB {
    generate_ray_attacks(sq, Direction::NE, occupied) |
    generate_ray_attacks(sq, Direction::NW, occupied) |
    generate_ray_attacks(sq, Direction::SE, occupied) |
    generate_ray_attacks(sq, Direction::SW, occupied)
}

fn generate_rook_attacks(sq: SQ, occupied: BB) -> BB {
    generate_ray_attacks(sq, Direction::N, occupied) |
    generate_ray_attacks(sq, Direction::S, occupied) |
    generate_ray_attacks(sq, Direction::E, occupied) |
    generate_ray_attacks(sq, Direction::W, occupied)
}

fn generate_queen_attacks(sq: SQ, occupied: BB) -> BB {
    generate_bishop_attacks(sq, occupied) |
    generate_rook_attacks(sq, occupied)
}

fn generate_king_attacks(sq: SQ) -> BB {
    let mut king_bb = BB::EMPTY;
    let rank = sq.rank();
    let file = sq.file();

    for d in ALL_DIRECTIONS.iter() {
        let new_sq_opt = sq.shift(d.file_offset(), d.rank_offset());
        if let Some(new_sq) = new_sq_opt{
            king_bb = king_bb.with_set(new_sq);
        }
    }
    king_bb
}

#[derive(Clone, Copy, Default)]
pub struct Magic {
    mask:   BB,
    mult:   BB,
    shift:  u8,
    offset: usize,
}

impl Magic {
    pub fn index(&self, occupied: BB) -> usize {
        let bb = ((occupied & self.mask) * self.mult) >> self.shift;
        self.offset as usize + bb.0 as usize
    }
}

pub fn write_magic(file: &mut std::fs::File) -> std::io::Result<()> {
    let magic_table  = generate_magic_table();
    let attack_table = generate_attack_table(&magic_table);
    let mut output = String::new();
    
    // Bishop magic table
    output.push_str("static BISHOP_MAGICS: [Magic; NUM_SQUARES] = [\n");
    for magic in magic_table.b_magics.iter() {
        output.push_str(&format!(
            "\tMagic {{ mask: BB({:#018X}), mult: BB({:#018X}), shift: {}, offset: {} }},\n",
            magic.mask.0, magic.mult.0, magic.shift, magic.offset
        ));
    }
    output.push_str("];\n\n");

    // Rook magic table
    output.push_str("static ROOK_MAGICS: [Magic; NUM_SQUARES] = [\n");
    for magic in magic_table.r_magics.iter() {
        output.push_str(&format!(
            "\tMagic {{ mask: BB({:#018X}), mult: BB({:#018X}), shift: {}, offset: {} }},\n",
            magic.mask.0, magic.mult.0, magic.shift, magic.offset
        ));
    }
    output.push_str("];\n\n");
    
    // Combined attack table for bishops and rooks
    output.push_str(&format!("const ATTACK_TABLE_SIZE: usize = {};\n", attack_table.len()));
    output.push_str("static ATTACK_TABLE: [BB; ATTACK_TABLE_SIZE] = [\n");
    for (i, attack) in attack_table.iter().enumerate() {
        if i % 4 == 0 {output.push_str("\t");}
        output.push_str(&format!("BB({:#018X}),", attack.0));
        if i % 4 == 3 {output.push_str("\n");}
    }
    output.push_str("];\n\n");

    file.write_all(output.as_bytes())?;
    println!("Magic tables written to magic_tables.txt");
    Ok(())
}



fn find_magic(sq: SQ, offset: usize, pt: PieceType) -> Magic {
    assert!(pt == PieceType::Bishop || pt == PieceType::Rook, "find_magic only supports bishops and rooks");

    let mut magic = Magic::default();
    
    magic.offset = offset;
        if pt == PieceType::Bishop {
            magic.mask  = generate_bishop_attacks(sq, BB::EMPTY) & !BB::EDGES;
        } else if pt == PieceType::Rook {
        let mut relevant_bits = !BB::EDGES;
        
        if sq.rank() == Rank::First {
            relevant_bits |= BB::from_rank(Rank::First);
        }

        if sq.rank() == Rank::Eighth {
            relevant_bits |= BB::from_rank(Rank::Eighth);
        }

        if sq.file() == File::A {
            relevant_bits |= BB::from_file(File::A);
        }

        if sq.file() == File::H {
            relevant_bits |= BB::from_file(File::H);
        }

        relevant_bits &= !BB::CORNERS;
        magic.mask = generate_rook_attacks(sq, BB::EMPTY) & relevant_bits;
    }

    magic.shift = (NUM_SQUARES - magic.mask.pop_count() as usize) as u8;
    
    let mut attack_table: [BB; 4096]  = [BB::EMPTY; 4096];
    let mut version:      [u64; 4096] = [0; 4096];
    let mut current_version: u64 = 0;
    let mut found_magic = false;
    
    if pt == PieceType::Bishop {
        print!("Finding bishop magic for {}...", sq);
    } else {
        print!("Finding rook magic for {}...", sq);
    }
    
    std::io::stdout().flush().expect("Failed to flush stdout");

    while found_magic == false {
        magic.mult = BB(rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>());
        let mut blockers = BB::EMPTY;

        loop {
            // Offset is removed to confine the index to within attack_table bounds.
            let index = magic.index(blockers) - magic.offset;

            let attacks = if pt == PieceType::Bishop {
                generate_bishop_attacks(sq, blockers)
            } else {
                generate_rook_attacks(sq, blockers)
            };
            
            if version[index] != current_version {
                attack_table[index] = attacks;
                version[index] = current_version;
            } else if attack_table[index] != attacks {
                break;
            }
            
            blockers = (blockers - magic.mask) & magic.mask;
            if blockers == BB::EMPTY {
                found_magic = true;
                break;
            }
        }

        current_version += 1;
    }

    println!("done.");
    magic
}

struct MagicTable {
    b_magics: [Magic; NUM_SQUARES],
    r_magics: [Magic; NUM_SQUARES],
    c: [Color; NUM_COLORS],
}

fn generate_magic_tables() -> MagicTable {
    let mut table = MagicTable {
        b_magics: [Magic::default(); NUM_SQUARES],
        r_magics: [Magic::default(); NUM_SQUARES],
     };
    let mut offset = 0;

    for sq in ALL_SQUARES.iter().copied() {
        table.b_magics[sq.to_index()] = find_magic(sq, offset, PieceType::Bishop);
        offset += 1 << (NUM_SQUARES - table.b_magics[sq.to_index()].shift as usize);
    }

    for sq in ALL_SQUARES.iter().copied() {
        table.r_magics[sq.to_index()] = find_magic(sq, offset, PieceType::Rook);
        offset += 1 << (NUM_SQUARES - table.r_magics[sq.to_index()].shift as usize);
    }

    table
}

fn generate_slider_attack_table(magic_table: &MagicTable) -> Vec<BB> {
    let size = magic_table.r_magics[NUM_SQUARES-1].offset + 
        (1 << (NUM_SQUARES - magic_table.r_magics[NUM_SQUARES-1].shift as usize));
    let mut attack_table = Vec::new();
    attack_table.resize(size, BB::EMPTY);

    println!("Attack table size: {} KB", size*8/1024);

    for (i, magic) in magic_table.b_magics.iter().enumerate() {
        let mut blockers = BB::EMPTY;
        loop {
            let index = magic.index(blockers);
            attack_table[index] = generate_bishop_attacks(SQ::from_index(i), blockers);
            blockers = (blockers - magic.mask) & magic.mask;
            if blockers == BB::EMPTY {
                break;
            }
        }
    }
    
    for (i, magic) in magic_table.r_magics.iter().enumerate() {
        let mut blockers = BB::EMPTY;
        loop {
            let index = magic.index(blockers);
            attack_table[index] = generate_rook_attacks(SQ::from_index(i), blockers);
            blockers = (blockers - magic.mask) & magic.mask;
            if blockers == BB::EMPTY {
                break;
            }
        }
    }

    attack_table
}