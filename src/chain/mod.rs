extern crate crypto;

use self::crypto::sha2::Sha256;
use self::crypto::digest::Digest;
use std;

pub type Nonce = u32;
pub type Hash = [u8; 32];
pub type Data = u64;

const ZERO_HASH: Hash = [0; 32];
const DIFFICULTY: u32 = 4;

pub struct Chain {
    pub blocks: Vec<Block>
}

#[repr(packed)]
pub struct Block {
    pub id: u64,
    pub nonce: Nonce,
    pub data: Data,
    pub prev_hash: Hash,
}

pub fn push(chain: &mut Chain, data: Data) {
    let new_block = make_block(chain.blocks.last(), data);
    info!("Added block to chain with id={}", new_block.id);
    chain.blocks.push(new_block);
}

pub fn init() -> Chain {
    info!("Creating new blockchain");
    Chain {
        blocks: Vec::new()
    }
}

pub fn verify(chain: &Chain) -> Result<(), String> {
    let mut i = 0;
    let mut prev_hash = ZERO_HASH;
    for block in &chain.blocks {
        if block.id != i {
            return Err(format!("Id mismatch at {} - expected {} but found {}", i, i, block.id));
        }
        if block.prev_hash != prev_hash {
            return Err(format!("Link broken at {} - expected hash {} but calculated {}", i, hash_to_str(&prev_hash), hash_to_str(&block.prev_hash)));
        }

        i += 1;
        prev_hash = hash(block);
    }

    Ok(())
}

pub fn print(chain: &Chain) {
    for block in &chain.blocks {
        let hash = hash(block);
        let prev_hash_str = hash_to_str(&block.prev_hash);
        let hash_str = hash_to_str(&hash);
        println!("Block {}: [nonce={}, data={}, prev={}, hash={}]", block.id, block.nonce, block.data, prev_hash_str, hash_str);
    }
}

fn hash_to_str(hash: &Hash) -> String {
    use std::fmt::Write;

    let mut s = String::new();
    for &byte in hash {
        write!(&mut s, "{:02x}", byte).unwrap();
    }
    s
}

fn make_block(prev: Option<&Block>, data: Data) -> Block {
    let prev_hash = prev.map(|b| hash(b)).unwrap_or(ZERO_HASH);
    let id = prev.map(|b| b.id + 1).unwrap_or(0);
    let mut block = Block {
        id: id,
        nonce: 0,
        data: data,
        prev_hash: prev_hash
    };

    find_nonce(&mut block, DIFFICULTY);
    block
}

fn find_nonce(block: &mut Block, difficulty: u32) {
    for n in 0..<u32>::max_value() {
        block.nonce = n;
        let hash = hash(block);
        if matches_difficulty(&hash, difficulty) {
            return;
        }
    }

    unreachable!("Unable to find nonce despite exhaustive search")
}

fn matches_difficulty(hash: &Hash, difficulty: u32) -> bool {
    assert!(hash.len() * 8 >= (difficulty as usize));

    leading_zero_bits(hash) >= difficulty
}

fn leading_zero_bits(hash: &Hash) -> u32 {
    let mut zero_bits = 0;
    for &byte in hash {
        if byte == 0 {
            zero_bits += 8;
        } else {
            zero_bits += byte.leading_zeros();
            break;
        }
    }

    zero_bits
}

fn hash(block: &Block) -> Hash {
    let data = as_bytes(block);
    let mut hasher = Sha256::new();
    hasher.input(data);

    let mut result = [0; 32];
    hasher.result(&mut result);
    result
}

fn as_bytes(block: & Block) -> & [u8] {
    let ptr = (block as *const Block) as *const u8;
    unsafe {
        std::slice::from_raw_parts(ptr, std::mem::size_of::<Block>())
    }
}
