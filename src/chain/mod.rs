extern crate crypto;
extern crate byteorder;

use std::fmt;
use self::crypto::sha2::Sha256;
use self::crypto::digest::Digest;
use self::byteorder::{BigEndian, WriteBytesExt};

pub type Nonce = u32;
pub type Hash = [u8; 32];
pub type Data = Vec<u8>;

const ZERO_HASH: Hash = [0; 32];
const DIFFICULTY: u32 = 4;

pub trait Blockchain {
    fn init() -> Self;
    fn push(&mut self, data: Data);
    fn verify(&self) -> Result<(), String>;
}

pub struct Chain {
    pub blocks: Vec<Block>
}

impl Blockchain for Chain {
    fn init() -> Self {
        init()
    }

    fn push(&mut self, data: Data) {
        push(self, data);
    }

    fn verify(&self) -> Result<(), String> {
        verify(self)
    }
}

impl fmt::Debug for Chain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chain:\n")?;
        for block in &self.blocks {
            let hash = hash(block);
            write!(f, "\t[id={}, nonce={}, data={}, prev={}, hash={}]\n", block.id, block.nonce,
                   bytes_to_str(&block.data), bytes_to_str(&block.prev_hash), bytes_to_str(&hash))?;
        }
        Ok(())
    }
}

pub struct Block {
    pub id: u64,
    pub nonce: Nonce,
    pub data: Data,
    pub prev_hash: Hash,
}

fn push(chain: &mut Chain, data: Data) {
    let new_block = make_block(chain.blocks.last(), data);
    info!("Added block to chain with id={}", new_block.id);
    chain.blocks.push(new_block);
}

fn init() -> Chain {
    info!("Creating new blockchain");
    Chain {
        blocks: Vec::new()
    }
}

fn verify(chain: &Chain) -> Result<(), String> {
    let mut prev_hash = ZERO_HASH;
    for (i, block) in chain.blocks.iter().enumerate() {
        if block.id != (i as u64) {
            return Err(format!("Id mismatch at {} - expected {} but found {}", i, i, block.id));
        }
        if block.prev_hash != prev_hash {
            return Err(format!("Link broken at {} - expected hash {} but calculated {}", i, bytes_to_str(&prev_hash), bytes_to_str(&block.prev_hash)));
        }

        let hash = hash(block);
        if !matches_difficulty(&hash, DIFFICULTY) {
            return Err(format!("Hash target failure at {} - expected target {} but hash was {}", i, DIFFICULTY, bytes_to_str(&hash)));
        }
        prev_hash = hash;
    }

    Ok(())
}

fn bytes_to_str(arr: &[u8]) -> String {
    use std::fmt::Write;

    let mut s = String::new();
    for &byte in arr {
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
    hasher.input(data.as_slice());

    let mut result = [0; 32];
    hasher.result(&mut result);
    result
}

fn as_bytes(block: &Block) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.write_u64::<BigEndian>(block.id).unwrap();
    bytes.write_u32::<BigEndian>(block.nonce).unwrap();
    bytes.extend(&block.data);
    bytes.extend_from_slice(&block.prev_hash);
    bytes
}

#[cfg(test)]
mod test {
    use super::{Blockchain, Chain, ZERO_HASH, find_nonce, hash};

    #[test]
    fn init_works() {
        let chain: Chain = Blockchain::init();
        assert_eq!(0, chain.blocks.len());
    }

    #[test]
    fn push_works() {
        let mut chain: Chain = Blockchain::init();

        chain.push(vec![0,0,0,0]);
        assert_eq!(1, chain.blocks.len());
        chain.push(vec![0,0,0,1]);
        assert_eq!(2, chain.blocks.len());

        let block1 = &chain.blocks[0];
        let block2 = &chain.blocks[1];
        assert_eq!(vec![0,0,0,0], block1.data);
        assert_eq!(ZERO_HASH, block1.prev_hash);
        assert_eq!(vec![0,0,0,1], block2.data);
        assert_eq!(hash(block1), block2.prev_hash);
    }

    #[test]
    fn verify_works() {
        let mut chain: Chain = Blockchain::init();

        chain.push(vec![1,2,3,4]);
        chain.push(vec![0]);
        chain.push(vec![5,6,7,8]);

        assert!(chain.verify().is_ok());
        chain.blocks[1].data = vec![5];
        assert!(chain.verify().is_err());
        chain.blocks[1].data = vec![0];
        assert!(chain.verify().is_ok());
        chain.blocks[2].prev_hash = ZERO_HASH;
        assert!(chain.verify().is_err());
        chain.blocks[2].prev_hash = hash(&chain.blocks[1]);
        assert!(chain.verify().is_ok());
        find_nonce(&mut chain.blocks[2], 1);
        assert!(chain.verify().is_err());
    }
}