extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn double_sha256(input: &Vec<u8>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    let mut out = [0; 32];
    hasher.input(input);
    hasher.result(&mut out);
    hasher.reset();
    hasher.input(&out);
    hasher.result(&mut out);
    out
}

pub fn sha256d(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    let mut out = [0; 32];
    hasher.input(input);
    hasher.result(&mut out);
    hasher.reset();
    hasher.input(&out);
    hasher.result(&mut out);
    out
}

#[cfg(test)]
mod tests {
    use crate::{double_sha256, sha256d};
    #[test]
    fn test_double_sha256() {
        assert_eq!(
            hex::encode(double_sha256(&b"hello".to_vec())),
            "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50"
        )
    }
    fn test_sha256d() {
        assert_eq!(
            hex::encode(sha256d(&b"hello".to_vec())),
            "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50"
        )
    }
}
