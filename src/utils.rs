use secp256k1::{rand::rngs, PublicKey, SecretKey, All, Secp256k1};
use secp256k1::rand::SeedableRng;
use tiny_keccak::{Hasher, Keccak};
pub fn generate_keypair(secp: &Secp256k1<All>, u64_seed: u64) -> (SecretKey, PublicKey) {
    let mut rng = rngs::StdRng::seed_from_u64(u64_seed);
    secp.generate_keypair(&mut rng)
}

pub fn from_hex(hex: &str, target: &mut [u8]) -> Result<usize, ()> {
    if hex.len() % 2 == 1 || hex.len() > target.len() * 2 {
        return Err(());
    }

    let mut b = 0;
    let mut idx = 0;
    for c in hex.bytes() {
        b <<= 4;
        match c {
            b'A'..=b'F' => b |= c - b'A' + 10,
            b'a'..=b'f' => b |= c - b'a' + 10,
            b'0'..=b'9' => b |= c - b'0',
            _ => return Err(()),
        }
        if (idx & 1) == 1 {
            target[idx / 2] = b;
            b = 0;
        }
        idx += 1;
    }
    Ok(idx / 2)
}

pub fn public_key_address_hash(public_key: &PublicKey, output: &mut[u8; 32]){
    let public_key = public_key.serialize_uncompressed();
    let mut hasher = Keccak::v256();
    hasher.update(&public_key[1..]);
    hasher.finalize(output);
}