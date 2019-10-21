type BlockHash = Vec<u8>;

use std::time::{ SystemTime, UNIX_EPOCH };

pub fn now() -> u128 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();

    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}

pub fn u32_bytes(u: &u32) -> [u8; 4] {
    [
        (u >> 8 * 0x0) as u8, // >> 0
        (u >> 8 * 0x1) as u8, // >> 8
        (u >> 8 * 0x2) as u8, // >> 16
        (u >> 8 * 0x3) as u8, // >> 24
    ]
}

pub fn u64_bytes(u: &u64) -> [u8; 8] {
    [
        (u >> 8 * 0x0) as u8, 
        (u >> 8 * 0x1) as u8, 
        (u >> 8 * 0x2) as u8, 
        (u >> 8 * 0x3) as u8, 
        
        (u >> 8 * 0x4) as u8, 
        (u >> 8 * 0x5) as u8, 
        (u >> 8 * 0x6) as u8, 
        (u >> 8 * 0x7) as u8, 
    ]
}

pub fn u128_bytes(u: &u128) -> [u8; 16] {
    [
        (u >> 8 * 0x0) as u8, 
        (u >> 8 * 0x1) as u8, 
        (u >> 8 * 0x2) as u8, 
        (u >> 8 * 0x3) as u8, 
        
        (u >> 8 * 0x4) as u8, 
        (u >> 8 * 0x5) as u8, 
        (u >> 8 * 0x6) as u8, 
        (u >> 8 * 0x7) as u8, 

        (u >> 8 * 0x8) as u8, 
        (u >> 8 * 0x9) as u8, 
        (u >> 8 * 0xa) as u8, 
        (u >> 8 * 0xb) as u8, 
        
        (u >> 8 * 0xc) as u8, 
        (u >> 8 * 0xd) as u8, 
        (u >> 8 * 0xe) as u8, 
        (u >> 8 * 0xf) as u8, 
    ]
}

// convert last 16 bytes to u128
pub fn difficulty_bytes_as_u128(v: &Vec<u8>) -> u128 {
    ((v[31] as u128) << 8 * 0xf) |
    ((v[30] as u128) << 8 * 0xe) |
    ((v[29] as u128) << 8 * 0xd) |
    ((v[28] as u128) << 8 * 0xc) |

    ((v[27] as u128) << 8 * 0xb) |
    ((v[26] as u128) << 8 * 0xa) |
    ((v[25] as u128) << 8 * 0x9) |
    ((v[24] as u128) << 8 * 0x8) |

    ((v[23] as u128) << 8 * 0x7) |
    ((v[22] as u128) << 8 * 0x6) |
    ((v[21] as u128) << 8 * 0x5) |
    ((v[20] as u128) << 8 * 0x4) |

    ((v[19] as u128) << 8 * 0x3) |
    ((v[18] as u128) << 8 * 0x2) |
    ((v[17] as u128) << 8 * 0x1) |
    ((v[16] as u128) << 8 * 0x0)
}

mod block;
pub use crate::block::Block;

mod hashable;
pub use crate::hashable::Hashable;

mod blockchain;
pub use crate::blockchain::BlockChain;

