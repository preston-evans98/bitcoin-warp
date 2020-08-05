extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

// pub fn double_sha256(input: &mut [u8]) {
//     let mut hasher = Sha256::new();
//     hasher.input(input);
//     hasher.result(input);
// }
pub fn double_sha256(input: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    let mut out = vec![0; 32];
    hasher.input(input);
    hasher.result(&mut out);
    hasher.reset();
    hasher.input(&out);
    hasher.result(&mut out);
    out
}
