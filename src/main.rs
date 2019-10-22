use rblockchainlib::*;



fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut block = Block::new(0, now(), vec![0; 32], 0, "First block".to_owned(), difficulty);

    block.mine();
    println!("Mined first block {:?}", &block);

    let mut last_hash = block.hash.clone();
    
    let mut blockchain = BlockChain {
        blocks: vec![block],
    };

    println!("Verify {}", &blockchain.verify());

    for i in 1..=10 {
        let mut block = Block::new(i, now(), last_hash, 0, "Another block".to_owned(), difficulty);

        block.mine();
        println!("Mined the block {:?}", &block);

        last_hash = block.hash.clone();

        blockchain.blocks.push(block);

        println!("Verify {}", &blockchain.verify());
    }
}
