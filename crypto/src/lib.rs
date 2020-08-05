extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

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

#[cfg(test)]
mod tests {
    use crate::double_sha256;
    #[test]
    fn test_double_sha256() {
        assert_eq!(
            hex::encode(double_sha256(&b"hello".to_vec())),
            "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50"
        )
    }
}
