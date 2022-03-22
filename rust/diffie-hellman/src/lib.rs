use num_bigint::BigUint;
use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let p = BigUint::from(p);
    let g = BigUint::from(g);
    let a = BigUint::from(a);

    g.modpow(&a, &p).iter_u64_digits().next().unwrap()
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    public_key(p, b_pub, a)
}
