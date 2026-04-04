
use std::io::Write;

use crate::board::{
    SQ, 
    File,
    Rank,
    BB,
    Direction
};

#[derive(Clone, Copy, Default)]
pub struct Magic {
    mask:   BB,
    mult:   BB,
    shift:  u8,
    offset: usize,
}

impl Magic {
    pub const fn index(&self, occupied: BB) -> usize {
        let bb = ((occupied.0 & self.mask.0).wrapping_mul(self.mult.0)) >> self.shift;
        self.offset + bb as usize
    }
}

pub const fn bishop_attacks(sq: SQ, blockers: BB) -> BB {
    let magic = BISHOP_MAGICS[sq.to_u8() as usize];
    ATTACK_TABLE[magic.index(blockers)]
}

pub const fn rook_attacks(sq: SQ, blockers: BB) -> BB {
    let magic = ROOK_MAGICS[sq.to_u8() as usize];
    ATTACK_TABLE[magic.index(blockers)]
}

const fn generate_ray_attacks(sq: SQ, d: Direction, blockers: BB) -> BB {
    let mut ray_bb = BB::EMPTY;
    let mut current_sq = sq;

    loop {
        let Some(next_sq) = current_sq.offset(d.file_offset(), d.rank_offset()) else {
            break;
        };

        current_sq = next_sq;
        ray_bb.0 |= BB::from_sq(current_sq).0; 

        if blockers.is_set(current_sq) {
            break; 
        }
    }

    ray_bb
}

pub const fn generate_bishop_attacks(sq: SQ, blockers: BB) -> BB {
    let bb_u64 = generate_ray_attacks(sq, Direction::NE, blockers).0 |
        generate_ray_attacks(sq, Direction::SE, blockers).0 |
        generate_ray_attacks(sq, Direction::SW, blockers).0 |
        generate_ray_attacks(sq, Direction::NW, blockers).0;
    BB(bb_u64)
}

pub const fn generate_rook_attacks(sq: SQ, blockers: BB) -> BB {
    let bb_u64 = generate_ray_attacks(sq, Direction::N, blockers).0 |
        generate_ray_attacks(sq, Direction::E, blockers).0 |
        generate_ray_attacks(sq, Direction::S, blockers).0 |
        generate_ray_attacks(sq, Direction::W, blockers).0;
    BB(bb_u64)
}

const fn generate_attack_table() -> [BB; ATTACK_TABLE_SIZE] {
    let mut table = [BB::EMPTY; ATTACK_TABLE_SIZE];
    let mut blockers = BB::EMPTY;
    let mut sq_index = 0;
    while sq_index < SQ::COUNT as usize {
        let sq = SQ::from_u8(sq_index as u8);
        table[BISHOP_MAGICS[sq_index].offset] = generate_bishop_attacks(sq, blockers);
        table[ROOK_MAGICS[sq_index].offset]   = generate_rook_attacks(sq, blockers);
        blockers = BB((blockers.0.wrapping_sub(BISHOP_MAGICS[sq_index].mask.0)) & BISHOP_MAGICS[sq_index].mask.0);
        if blockers.0 == 0 {
            break;
        }
        sq_index += 1;
    }
    
    table
}

const ATTACK_TABLE_SIZE: usize = 107648;
#[allow(long_running_const_eval)]
static ATTACK_TABLE: [BB; ATTACK_TABLE_SIZE] = generate_attack_table();
static BISHOP_MAGICS: [Magic; SQ::COUNT as usize] = [
	Magic { mask: BB(0x0040201008040200), mult: BB(0x0010709002808012), shift: 58, offset: 0 },
	Magic { mask: BB(0x0000402010080400), mult: BB(0x0020840C20404000), shift: 59, offset: 64 },
	Magic { mask: BB(0x0000004020100A00), mult: BB(0x1004010401000000), shift: 59, offset: 96 },
	Magic { mask: BB(0x0000000040221400), mult: BB(0xA014040080008000), shift: 59, offset: 128 },
	Magic { mask: BB(0x0000000002442800), mult: BB(0x0002021000010120), shift: 59, offset: 160 },
	Magic { mask: BB(0x0000000204085000), mult: BB(0x0200822020240201), shift: 59, offset: 192 },
	Magic { mask: BB(0x0000020408102000), mult: BB(0x2012020104C00000), shift: 59, offset: 224 },
	Magic { mask: BB(0x0002040810204000), mult: BB(0x8000840041142000), shift: 58, offset: 256 },
	Magic { mask: BB(0x0020100804020000), mult: BB(0x0010111001080080), shift: 59, offset: 320 },
	Magic { mask: BB(0x0040201008040000), mult: BB(0x8000040808010020), shift: 59, offset: 352 },
	Magic { mask: BB(0x00004020100A0000), mult: BB(0x0010080084008000), shift: 59, offset: 384 },
	Magic { mask: BB(0x0000004022140000), mult: BB(0x2082040400803020), shift: 59, offset: 416 },
	Magic { mask: BB(0x0000000244280000), mult: BB(0x0800120210400042), shift: 59, offset: 448 },
	Magic { mask: BB(0x0000020408500000), mult: BB(0x0000009010080000), shift: 59, offset: 480 },
	Magic { mask: BB(0x0002040810200000), mult: BB(0x20000200840C4030), shift: 59, offset: 512 },
	Magic { mask: BB(0x0004081020400000), mult: BB(0x040002004A021000), shift: 59, offset: 544 },
	Magic { mask: BB(0x0010080402000200), mult: BB(0x000C221090100100), shift: 59, offset: 576 },
	Magic { mask: BB(0x0020100804000400), mult: BB(0x000C000238020400), shift: 59, offset: 608 },
	Magic { mask: BB(0x004020100A000A00), mult: BB(0x0011000206020200), shift: 57, offset: 640 },
	Magic { mask: BB(0x0000402214001400), mult: BB(0x0042200802004100), shift: 57, offset: 768 },
	Magic { mask: BB(0x0000024428002800), mult: BB(0x9004000080A00010), shift: 57, offset: 896 },
	Magic { mask: BB(0x0002040850005000), mult: BB(0x0001000200860500), shift: 57, offset: 1024 },
	Magic { mask: BB(0x0004081020002000), mult: BB(0x0808400108223042), shift: 59, offset: 1152 },
	Magic { mask: BB(0x0008102040004000), mult: BB(0x1028820034010800), shift: 59, offset: 1184 },
	Magic { mask: BB(0x0008040200020400), mult: BB(0x050C200010210100), shift: 59, offset: 1216 },
	Magic { mask: BB(0x0010080400040800), mult: BB(0x0002020020280200), shift: 59, offset: 1248 },
	Magic { mask: BB(0x0020100A000A1000), mult: BB(0x0082280004080020), shift: 57, offset: 1280 },
	Magic { mask: BB(0x0040221400142200), mult: BB(0x1040040090410020), shift: 55, offset: 1408 },
	Magic { mask: BB(0x0002442800284400), mult: BB(0x000101003010C000), shift: 55, offset: 1920 },
	Magic { mask: BB(0x0004085000500800), mult: BB(0x081000C002080200), shift: 57, offset: 2432 },
	Magic { mask: BB(0x0008102000201000), mult: BB(0x0008010200808880), shift: 59, offset: 2560 },
	Magic { mask: BB(0x0010204000402000), mult: BB(0x0001010000C04808), shift: 59, offset: 2592 },
	Magic { mask: BB(0x0004020002040800), mult: BB(0x0002024044101000), shift: 59, offset: 2624 },
	Magic { mask: BB(0x0008040004081000), mult: BB(0x0901820844101080), shift: 59, offset: 2656 },
	Magic { mask: BB(0x00100A000A102000), mult: BB(0x0000404040180200), shift: 57, offset: 2688 },
	Magic { mask: BB(0x0022140014224000), mult: BB(0x80000A0280080080), shift: 55, offset: 2816 },
	Magic { mask: BB(0x0044280028440200), mult: BB(0x4002410042140040), shift: 55, offset: 3328 },
	Magic { mask: BB(0x0008500050080400), mult: BB(0x4010004040020104), shift: 57, offset: 3840 },
	Magic { mask: BB(0x0010200020100800), mult: BB(0x40020082004C0200), shift: 59, offset: 3968 },
	Magic { mask: BB(0x0020400040201000), mult: BB(0x0018010029A10882), shift: 59, offset: 4000 },
	Magic { mask: BB(0x0002000204081000), mult: BB(0x0000821110004028), shift: 59, offset: 4032 },
	Magic { mask: BB(0x0004000408102000), mult: BB(0x0401808808002180), shift: 59, offset: 4064 },
	Magic { mask: BB(0x000A000A10204000), mult: BB(0x000100118200B008), shift: 57, offset: 4096 },
	Magic { mask: BB(0x0014001422400000), mult: BB(0x000040A011000800), shift: 57, offset: 4224 },
	Magic { mask: BB(0x0028002844020000), mult: BB(0x0001180100400400), shift: 57, offset: 4352 },
	Magic { mask: BB(0x0050005008040200), mult: BB(0x8801100100404200), shift: 57, offset: 4480 },
	Magic { mask: BB(0x0020002010080400), mult: BB(0x0010A40100420400), shift: 59, offset: 4608 },
	Magic { mask: BB(0x0040004020100800), mult: BB(0x0008520420400420), shift: 59, offset: 4640 },
	Magic { mask: BB(0x0000020408102000), mult: BB(0x9009480444200000), shift: 59, offset: 4672 },
	Magic { mask: BB(0x0000040810204000), mult: BB(0x0212010402420000), shift: 59, offset: 4704 },
	Magic { mask: BB(0x00000A1020400000), mult: BB(0x0012048400884040), shift: 59, offset: 4736 },
	Magic { mask: BB(0x0000142240000000), mult: BB(0x0000000084040000), shift: 59, offset: 4768 },
	Magic { mask: BB(0x0000284402000000), mult: BB(0x00008040304D0200), shift: 59, offset: 4800 },
	Magic { mask: BB(0x0000500804020000), mult: BB(0x0800100210010000), shift: 59, offset: 4832 },
	Magic { mask: BB(0x0000201008040200), mult: BB(0x0020200610810000), shift: 59, offset: 4864 },
	Magic { mask: BB(0x0000402010080400), mult: BB(0x1102022424008190), shift: 59, offset: 4896 },
	Magic { mask: BB(0x0002040810204000), mult: BB(0x0802010088010801), shift: 58, offset: 4928 },
	Magic { mask: BB(0x0004081020400000), mult: BB(0x000000208C100800), shift: 59, offset: 4992 },
	Magic { mask: BB(0x000A102040000000), mult: BB(0x0008080021080808), shift: 59, offset: 5024 },
	Magic { mask: BB(0x0014224000000000), mult: BB(0x0000000000208800), shift: 59, offset: 5056 },
	Magic { mask: BB(0x0028440200000000), mult: BB(0x2200000030820202), shift: 59, offset: 5088 },
	Magic { mask: BB(0x0050080402000000), mult: BB(0x61008020A0020080), shift: 59, offset: 5120 },
	Magic { mask: BB(0x0020100804020000), mult: BB(0x0000202021020081), shift: 59, offset: 5152 },
	Magic { mask: BB(0x0040201008040200), mult: BB(0x0020085000808410), shift: 58, offset: 5184 },
];

static ROOK_MAGICS: [Magic; SQ::COUNT as usize] = [
	Magic { mask: BB(0x000101010101017E), mult: BB(0x8280004004802010), shift: 52, offset: 5248 },
	Magic { mask: BB(0x000202020202027C), mult: BB(0x0040004020001004), shift: 53, offset: 9344 },
	Magic { mask: BB(0x000404040404047A), mult: BB(0x310011C101882000), shift: 53, offset: 11392 },
	Magic { mask: BB(0x0008080808080876), mult: BB(0x0880100008000580), shift: 53, offset: 13440 },
	Magic { mask: BB(0x001010101010106E), mult: BB(0x0100030008001004), shift: 53, offset: 15488 },
	Magic { mask: BB(0x002020202020205E), mult: BB(0x02000200381C0110), shift: 53, offset: 17536 },
	Magic { mask: BB(0x004040404040403E), mult: BB(0x0080008022002100), shift: 53, offset: 19584 },
	Magic { mask: BB(0x008080808080807E), mult: BB(0x0080102041000080), shift: 52, offset: 21632 },
	Magic { mask: BB(0x0001010101017E00), mult: BB(0x0100800064400080), shift: 53, offset: 25728 },
	Magic { mask: BB(0x0002020202027C00), mult: BB(0x0000400820100040), shift: 54, offset: 27776 },
	Magic { mask: BB(0x0004040404047A00), mult: BB(0x0000801004842004), shift: 54, offset: 28800 },
	Magic { mask: BB(0x0008080808087600), mult: BB(0x0000808010000800), shift: 54, offset: 29824 },
	Magic { mask: BB(0x0010101010106E00), mult: BB(0x0000800C00280081), shift: 54, offset: 30848 },
	Magic { mask: BB(0x0020202020205E00), mult: BB(0x8008800400020080), shift: 54, offset: 31872 },
	Magic { mask: BB(0x0040404040403E00), mult: BB(0x0001008200040100), shift: 54, offset: 32896 },
	Magic { mask: BB(0x0080808080807E00), mult: BB(0x0000800049002080), shift: 53, offset: 33920 },
	Magic { mask: BB(0x00010101017E0100), mult: BB(0x0280004020024001), shift: 53, offset: 35968 },
	Magic { mask: BB(0x00020202027C0200), mult: BB(0x0020008021400090), shift: 54, offset: 38016 },
	Magic { mask: BB(0x00040404047A0400), mult: BB(0x0200820018420020), shift: 54, offset: 39040 },
	Magic { mask: BB(0x0008080808760800), mult: BB(0x0000808010028800), shift: 54, offset: 40064 },
	Magic { mask: BB(0x00101010106E1000), mult: BB(0x0801010004080010), shift: 54, offset: 41088 },
	Magic { mask: BB(0x00202020205E2000), mult: BB(0x2000808004000200), shift: 54, offset: 42112 },
	Magic { mask: BB(0x00404040403E4000), mult: BB(0x8000040002081001), shift: 54, offset: 43136 },
	Magic { mask: BB(0x00808080807E8000), mult: BB(0x1000020001884401), shift: 53, offset: 44160 },
	Magic { mask: BB(0x000101017E010100), mult: BB(0x0080004140102000), shift: 53, offset: 46208 },
	Magic { mask: BB(0x000202027C020200), mult: BB(0x0080400440201000), shift: 54, offset: 48256 },
	Magic { mask: BB(0x000404047A040400), mult: BB(0x4000420200102080), shift: 54, offset: 49280 },
	Magic { mask: BB(0x0008080876080800), mult: BB(0x0102100080800800), shift: 54, offset: 50304 },
	Magic { mask: BB(0x001010106E101000), mult: BB(0x0114008080080004), shift: 54, offset: 51328 },
	Magic { mask: BB(0x002020205E202000), mult: BB(0x0000840080800200), shift: 54, offset: 52352 },
	Magic { mask: BB(0x004040403E404000), mult: BB(0x0002000200080401), shift: 54, offset: 53376 },
	Magic { mask: BB(0x008080807E808000), mult: BB(0x0000800380004100), shift: 53, offset: 54400 },
	Magic { mask: BB(0x0001017E01010100), mult: BB(0x0200804000800020), shift: 53, offset: 56448 },
	Magic { mask: BB(0x0002027C02020200), mult: BB(0x0000802008804000), shift: 54, offset: 58496 },
	Magic { mask: BB(0x0004047A04040400), mult: BB(0x0000802000803000), shift: 54, offset: 59520 },
	Magic { mask: BB(0x0008087608080800), mult: BB(0x4288001000808008), shift: 54, offset: 60544 },
	Magic { mask: BB(0x0010106E10101000), mult: BB(0x0090041101000800), shift: 54, offset: 61568 },
	Magic { mask: BB(0x0020205E20202000), mult: BB(0x0000020080800400), shift: 54, offset: 62592 },
	Magic { mask: BB(0x0040403E40404000), mult: BB(0x0000085044004201), shift: 54, offset: 63616 },
	Magic { mask: BB(0x0080807E80808000), mult: BB(0x0200800040800100), shift: 53, offset: 64640 },
	Magic { mask: BB(0x00017E0101010100), mult: BB(0x0400400180008020), shift: 53, offset: 66688 },
	Magic { mask: BB(0x00027C0202020200), mult: BB(0x4042500120044000), shift: 54, offset: 68736 },
	Magic { mask: BB(0x00047A0404040400), mult: BB(0x8020004100110028), shift: 54, offset: 69760 },
	Magic { mask: BB(0x0008760808080800), mult: BB(0x0208002010010100), shift: 54, offset: 70784 },
	Magic { mask: BB(0x00106E1010101000), mult: BB(0x1008002040040400), shift: 54, offset: 71808 },
	Magic { mask: BB(0x00205E2020202000), mult: BB(0x0002000804020010), shift: 54, offset: 72832 },
	Magic { mask: BB(0x00403E4040404000), mult: BB(0x0000C80170040002), shift: 54, offset: 73856 },
	Magic { mask: BB(0x00807E8080808000), mult: BB(0x0000040068820011), shift: 53, offset: 74880 },
	Magic { mask: BB(0x007E010101010100), mult: BB(0x0020230890420200), shift: 53, offset: 76928 },
	Magic { mask: BB(0x007C020202020200), mult: BB(0x0001201000400240), shift: 54, offset: 78976 },
	Magic { mask: BB(0x007A040404040400), mult: BB(0x0010040020080020), shift: 54, offset: 80000 },
	Magic { mask: BB(0x0076080808080800), mult: BB(0x4000821000080080), shift: 54, offset: 81024 },
	Magic { mask: BB(0x006E101010101000), mult: BB(0x00A08008000C0280), shift: 54, offset: 82048 },
	Magic { mask: BB(0x005E202020202000), mult: BB(0x0128040080020080), shift: 54, offset: 83072 },
	Magic { mask: BB(0x003E404040404000), mult: BB(0x0002000108040200), shift: 54, offset: 84096 },
	Magic { mask: BB(0x007E808080808000), mult: BB(0x2008800041001080), shift: 53, offset: 85120 },
	Magic { mask: BB(0x7E01010101010100), mult: BB(0x0020800021005041), shift: 52, offset: 87168 },
	Magic { mask: BB(0x7C02020202020200), mult: BB(0x0200208040001105), shift: 53, offset: 91264 },
	Magic { mask: BB(0x7A04040404040400), mult: BB(0x0008104102200009), shift: 53, offset: 93312 },
	Magic { mask: BB(0x7608080808080800), mult: BB(0x1001000C20100009), shift: 53, offset: 95360 },
	Magic { mask: BB(0x6E10101010101000), mult: BB(0x000100101800048F), shift: 53, offset: 97408 },
	Magic { mask: BB(0x5E20202020202000), mult: BB(0x0101000806440001), shift: 53, offset: 99456 },
	Magic { mask: BB(0x3E40404040404000), mult: BB(0x0005000404820001), shift: 53, offset: 101504 },
	Magic { mask: BB(0x7E80808080808000), mult: BB(0x0002040020814102), shift: 52, offset: 103552 },
];

 
