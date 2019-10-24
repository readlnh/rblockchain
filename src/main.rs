use rblockchainlib::*;



fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "readlnh".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "tangtangtang".to_owned(),
                    value: 7,
                }
            ]
        }
    ], difficulty);

    genesis_block.mine();
    println!("Mined first block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();
    
    let mut blockchain = BlockChain::new();
    
    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");
    

    
}
