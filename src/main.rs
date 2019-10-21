use rblockchainlib::*;



fn main() {
    let mut block = Block::new(0, now(), vec![0; 32], 0, "First block".to_owned(), 0x00000fffffffffffffffffffffffffff);
    println!("{:?}", &block);


    block.mine();
    println!("{:?}", &block);
}
