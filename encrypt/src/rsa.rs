extern crate rand;
use rand::Rng;
pub const LIMIT: i64 = 10000;
pub struct RSA {
    pub_key: (i64, i64),
    priv_key: (i64, i64),
}
impl RSA {
    pub fn new() -> Self {
        let (pub_key, priv_key) = Self::gen_paire_key();
        RSA { pub_key, priv_key }
    }
    pub fn get_pub_key(&self) -> (i64, i64) {
        self.pub_key
    }
    pub fn get_priv_key(&self) -> (i64, i64) {
        self.priv_key
    }
    pub fn decrypt(key: (i64, i64), value: i64) -> i64 {
        let (m, p) = key;
        Self::log_power(value, p, m)
    }
    pub fn encrypt(key: (i64, i64), value: i64) -> i64 {
        let (m, p) = key;
        Self::log_power(value, p, m)
    }

    fn gen_paire_key() -> ((i64, i64), (i64, i64)) {
        let (p, q) = (Self::gen_prime(), Self::gen_prime());
        let (n, phi) = (p * q, (p - 1) * (q - 1));
        let e = Self::gen_coprime(phi);
        let d = Self::modular_inverse(e, phi);
        ((n, e), (n, d))
    }
    fn modular_inverse(n:i64, m: i64) -> i64 {
       let (mut inverse,_)=Self::euclid_extended(n,m);
       while inverse<0{
           inverse+=m;
       }
       inverse
    }
    fn rabin_miller(n: i64) -> bool {
        let mut res=true;
        for _i in 0..5{
            if !res{
                return res;
            }
            let a=Self::gen_rand()+1;
            let l=Self::log_power(a,n-1,n);
            res&=l==1;
        }
        res
    }
    fn euclid_extended(a: i64, b: i64) -> (i64, i64) {
        if b == 0 {
            (1, 0)
        } else {
            let (c, d) = Self::euclid_extended(b, a % b);

            (d, c - (a / b) * d)
        }
    }
    fn log_power(mut n: i64, mut p: i64, m: i64) -> i64 {
        let mut result = 1;
        loop {
            if p == 0 {
                break;
            }
            if p & 1 != 0 {
                result = (result * n) % m;
            }
            n = (n * n) % m;
            p >>= 1;
        }
        result
    }
    fn gen_rand() -> i64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(2, LIMIT)
    }
    fn gen_coprime(n: i64) -> i64 {
        let mut generated = Self::gen_rand();
        while Self::gcd(generated, n) != 1 {
            generated = Self::gen_rand();
        }
        return generated;
    }
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        return a;
    }
    fn gen_prime() -> i64 {
        let mut generated = Self::gen_rand();
        while !Self::rabin_miller(generated) {
            generated = Self::gen_rand();
        }
        generated
    }
}
