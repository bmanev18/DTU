use bnum::types::U256;

struct PublicKey {
    e: u128,
    n: u128,
}

struct PrivateKey {
    d: u128,
    n: u128,
}

fn phi(p: u128, q: u128) -> Option<U256> {
    let one = U256::ONE;
    (U256::from(p) - one).checked_mul(U256::from(q) - one)
}

fn key_gen() {
    let p = 3;
    let q = 5;

    let phi = phi(p, q);
}

fn main() {
    let a = U256::from(1u32);
    // let b = a - 1

    println!("Hello, world!");
}
