mod utils;

use std::time::Instant;
use clap::Parser;
use secp256k1::{All, Secp256k1};
use crate::utils::from_hex;


/// A simple wallet cracker using time stamp as seed
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {

    /// Lower search bounds
    #[arg(
        short = 'l',
        long="lower",
        long_help="Lower bounds for time seed search, default is July 30:th 2015 in milli seconds (time of ETH Blockchain going live)",
        // default_value = "1438207200000"
    )]
    lower: u64,

    /// Upper search bounds
    #[arg(
        short = 'u',
        long="upper",
        long_help="Upper bounds for time seed search, recommended is to narrow it down as much as possible for fastest search times"
    )]
    upper: u64,

    #[arg(short = 'a', long="address")]
    /// Target ETH address WITHOUT leading 0x
    address: String,
}

fn run_main(lower: u64, upper: u64, address: &[u8]) {
    // Initialize as few variables as possible, minimizing execution overhead in for loop
    let secp: Secp256k1<All> = secp256k1::Secp256k1::new();
    let now = Instant::now();
    let mut output= [0u8; 32];
    for seed in lower..upper {
        let (priv_key, pub_key) = utils::generate_keypair(&secp, seed);
        utils::public_key_address_hash(&pub_key, &mut output);
        let public_address = &output[12..];
        if public_address == address {
            println!("Found (ADDRESS,KEY,TRIES) {:?}:{}:{}", address, &priv_key.to_string(), seed - lower);
            break
        }
    }

    let total = now.elapsed().as_micros();
    println!("Tested in {} microseconds", total);

}

fn main() {
    let args = Args::parse();
    let lower: u64 = args.lower;
    let upper: u64 = args.upper;
    let mut address: [u8; 20] = [0u8;20];
    from_hex(&*args.address.to_lowercase(), &mut address).unwrap();
    run_main(lower, upper, &address);
}

#[cfg(test)]
    #[test]
    fn test_public_address() {
        use std::time::Instant;
        let seed = 1711291285; // created time using https://github.com/tmsdev82/rust-eth-crypto-wallet-tutorial/commit/e10f7b5eb99b4a16d2f64cffab48bb4da5ec6242

    let secp: Secp256k1<All> = secp256k1::Secp256k1::new();
        let (_, pub_key) = utils::generate_keypair(&secp, seed);
        let now = Instant::now();
        let mut output= [0u8; 32];
        utils::public_key_address_hash(&pub_key, &mut output);
        let public_address = &output[12..];
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?} {:?}", elapsed, public_address);
    }

    #[test]
    fn test_from_hex() {
        let test_address = "5561cabc7185c87cc036637ead7bc9efc6ef9f55";
        let mut result: [u8; 20] = [0u8;20];
        let len  = from_hex(test_address, &mut result).unwrap();
        println!("Elapsed: {:?} {:?} ", result, len);
    }