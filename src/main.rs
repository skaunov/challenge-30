use md4::{Md4Core, Md4, Digest, digest::core_api::CoreWrapper};
use rand::{RngCore, rngs::OsRng};

fn main() {

    // ## create "key"
    let ranum = OsRng.next_u32();
    let size_ran: usize = (3 + ranum % 4) as usize;
    let mut secret_a:Vec<u8> = vec![0; size_ran];
    OsRng.fill_bytes(secret_a.as_mut_slice());
    // let secret_len = secret_a.len();
    // println!("`secret_a`:{secret_len}|{secret_a:?}");

    let mut hasher = Md4::new();
    hasher.update(&secret_a);
    hasher.update(b"comment1=cooking%20MCs;userdata=foo;comment2=%20like%20a%20pound%20of%20bacon");
    let mes_orig = hasher.finalize();
    println!("mes_orig:{mes_orig:?}");

    let mut hasher = Md4::new();
    hasher.update(padding_md(&[secret_a, b"comment1=cooking%20MCs;userdata=foo;comment2=%20like%20a%20pound%20of%20bacon".to_vec()].concat()));
    hasher.update(b";admin=true");
    let mes_target = hasher.finalize();
    println!("mes_target:{mes_target:?}");

    let hasher_core = Md4Core{block_len: 2, state: [
        u32::from_le_bytes(mes_orig[0..=3].try_into().unwrap()),
        u32::from_le_bytes(mes_orig[4..=7].try_into().unwrap()),
        u32::from_le_bytes(mes_orig[8..=11].try_into().unwrap()),
        u32::from_le_bytes(mes_orig[12..=15].try_into().unwrap())
    ]};
    let mut hasher = CoreWrapper::from_core(hasher_core);
    hasher.update(";admin=true");
    let mes_solut = hasher.finalize();
    println!("mes_solut:{mes_solut:?}");
}

fn padding_md(md: &[u8]) -> Vec<u8> {
    let padded = [md, [128 as u8].as_slice()].concat();
    let zeroes: Vec<u8> = vec![0; 63 - 8 - md.len() % 64];
    let mut padded = [padded, zeroes].concat();
    let last_bytes = (8 * md.len()).to_le_bytes();
    padded = [padded, last_bytes.to_vec()].concat();
    println!("`padding_md` result: {padded:?}");
    assert!(padded.len() % 64 == 0);
    padded
}