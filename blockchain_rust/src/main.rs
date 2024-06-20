use sha2::{Sha256, Digest};

// Define the Block structure
struct Block {
    prev_hash: Vec<u8>,
    hash: Vec<u8>,
    data: Vec<u8>,
}

// Define the BlockChain structure
struct BlockChain {
    blocks: Vec<Block>,
}

impl Block {
    // Function to compute and set the hash for a block
    fn set_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(&self.prev_hash);
        hasher.update(&self.data);
        self.hash = hasher.finalize().to_vec();
    }

    // Create a new block
    fn new(data: &str, prev_hash: Vec<u8>) -> Block {
        let mut block = Block {
            prev_hash,
            hash: Vec::new(),
            data: data.as_bytes().to_vec(),
        };
        block.set_hash();
        block
    }
}

impl BlockChain {
    // Function to create a new blockchain with the genesis block
    fn new() -> BlockChain {
        let genesis_data = "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";
        let genesis_block = Block::new(genesis_data, vec![0x00]);
        BlockChain { blocks: vec![genesis_block] }
    }
}

fn main() {
    let bc = BlockChain::new();
    for (i, block) in bc.blocks.iter().enumerate() {
        println!("======== block height : {} =======", i);
        println!("PreHash : {:x?}", block.prev_hash);
        // 将每个字节转换为十六进制字符串
        let hex_string: String = block.hash.iter()
            .map(|byte| format!("{:02x}", byte))  // 将每个字节格式化为两位十六进制数
            .collect();  // 拼接成一个完整的字符串
        println!("Hash : {}", hex_string);
        println!("Data : {}", String::from_utf8_lossy(&block.data));
    }
}
