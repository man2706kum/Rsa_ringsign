use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, NewBlockCipher};
use aes::{Aes128, Block};
use byteorder::{BigEndian, ReadBytesExt};
use rsa::BigUint;

#[allow(dead_code)]
pub fn encrypt(key: BigUint, m: BigUint) -> BigUint {
    let slices = key.to_bytes_be();
    let text = m.to_bytes_be();
    //println!("Enc:{:?} {:?}", slices.len(), text.len());
    let key = GenericArray::from_slice(&slices);
    let block = Block::from_slice(&text);

    let mut block_test = block.clone();
    // Initialize cipher
    let cipher = Aes128::new(&key);
    // Encrypt block in-place
    cipher.encrypt_block(&mut block_test);
    let mut x = block_test.as_slice();

    let num = x.read_u128::<BigEndian>().unwrap();
    return BigUint::from(num);
}

#[allow(dead_code)]
pub fn decrypt(key: BigUint, m: BigUint) -> BigUint {
    let slices = key.to_bytes_be();
    let text = m.to_bytes_be();
    //println!("Dec:{:?} {:?}", slices.len(), text.len());

    let key = GenericArray::from_slice(&slices);
    let block = Block::from_slice(&text);

    let mut block_test = block.clone();
    // Initialize cipher
    let cipher = Aes128::new(&key);
    // Encrypt block in-place
    cipher.decrypt_block(&mut block_test);
    let mut x = block_test.as_slice();

    let num = x.read_u128::<BigEndian>().unwrap();
    return BigUint::from(num);
}
/*
#[allow(dead_code)]
pub fn encrypt256(key: BigUint, m: BigUint) {
    let slices = key.to_bytes_be();
    let mut text = m.to_bytes_be();
    //println!("Enc:{:?} {:?}", slices.len(), text.len());
    let key = GenericArray::from_slice(&slices);
    let texts = GenericArray::from_slice(&text);
    let block = ParBlocks::from_slice(texts.as_slice());

    let mut block_test = block.clone();
    // Initialize cipher
    let cipher = Aes128::new(&key);
    // Encrypt block in-place
    cipher.encrypt_par_blocks(&mut block_test);
    let mut x = block_test.as_slice();
    println!("{:?}", x);
}
*/